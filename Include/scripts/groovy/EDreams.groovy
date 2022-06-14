import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import java.text.SimpleDateFormat

import org.openqa.selenium.Keys

import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import com.travelport.testfactory.*

import cucumber.api.java.en.And
import cucumber.api.java.en.Given
import cucumber.api.java.en.Then
import cucumber.api.java.en.When
import groovy.json.JsonOutput
import groovy.json.JsonSlurper
import internal.GlobalVariable



class EDreams {
	/**
	 * The step definitions below match with Katalon sample Gherkin steps
	 */
	static def PriceRsp
	static def AirBookRsp
	static def URRetrieveRsp
	static def UR
	static def Response2
	def slurper = new groovy.json.JsonSlurper()
	static String xml2 = ""
	def jsonSlurper = new JsonSlurper()

	@Given("OAUTH Token for PCC (.*) is generated")
	def generate_oauth_token_for_pcc(String pcc) {
		println "TripService Generate OAUTH Token for PCC" + pcc
		Extbase.setManualProxy()
		def oauthCredentialsList = jsonSlurper.parseText(GlobalVariable.OauthCredentialsList)

		for(def oauthAppInfo : oauthCredentialsList.oauthAppInfoList) {
			if(oauthAppInfo.pcc.equalsIgnoreCase(pcc)) {
				GlobalVariable.OauthUsername = oauthAppInfo.userName
				GlobalVariable.OauthPassword = oauthAppInfo.password
				GlobalVariable.OauthClientId = oauthAppInfo.clientId
				GlobalVariable.OauthClientSecret = oauthAppInfo.clientSecret
				break;
			}
		}

		//def oAuthResponse = WS.sendRequestAndVerify(findTestObject('Trip Services/OAuth'))
		def oAuthResponse = WS.sendRequestAndVerify(findTestObject('Trip Services/OAuth - Dynamic'))
		WS.verifyResponseStatusCode(oAuthResponse, 200)
	}

	@And("the Trip Type is (.*) OD is (.*) Departure Date is (.*) Pax Type is (.*)")
	def add_triptype_od_departuredate(String tripType, String fromTo, String departureDates, String paxType) {
		def searchPayloadFormation = jsonSlurper.parseText(GlobalVariable.Search_Payload_V9)
		def getFromToDetailList = fromTo.split("-")
		def departureDate = departureDates.split(",")
		def i = 0
		if(tripType.contains("Oneway")) {
			addSearchCriteriaFlight(getFromToDetailList, i, departureDate, searchPayloadFormation)
		} else if(tripType.contains("RoundTrip")) {
			for(i=0;i<2;i++) {
				addSearchCriteriaFlight(getFromToDetailList, i, departureDate, searchPayloadFormation)
			}
		} else if(tripType.contains("CircularTrip")||tripType.contains("OpenJaw")) {
			for(i=0;i<3;i++) {
				addSearchCriteriaFlight(getFromToDetailList, i, departureDate, searchPayloadFormation)
			}
		}

		def jsonSearchPayloadFormation = new JsonOutput().toJson(searchPayloadFormation)
		GlobalVariable.Search_Payload_V9 = jsonSearchPayloadFormation

		setPassengerCriteria(paxType)
	}

	def addSearchCriteriaFlight(String[] getFromToDetailList, int i, String[] departureDate, searchPayloadFormation) {
		def searchCriteriaFlight_Payload = jsonSlurper.parseText('{"departureDate": "2021-12-20","From": {"value": "MEL"},"To": {"value": "SYD"}}')
		searchCriteriaFlight_Payload.From.value = getFromToDetailList[i]
		searchCriteriaFlight_Payload.To.value = getFromToDetailList[i+1]
		searchCriteriaFlight_Payload.departureDate = getFeatureDate(departureDate[i])
		searchPayloadFormation.CatalogOfferingsQueryRequest.CatalogOfferingsRequest[0].SearchCriteriaFlight.add(searchCriteriaFlight_Payload)
	}

	def getFeatureDate(String addDay){
		int addDays = addDay.toInteger()
		def dateFormat = new SimpleDateFormat("yyyy-MM-dd");
		Calendar c = Calendar.getInstance();
		c.add(Calendar.DATE, addDays);
		String futureDate = dateFormat.format(c.getTime());
		return futureDate;
	}

	def setPassengerCriteria(String paxType) {
		def getPaxTypeList = paxType.split(",")
		def searchPayloadFormation = jsonSlurper.parseText(GlobalVariable.Search_Payload_V9)

		for (int l = 0; l < getPaxTypeList.size(); l++) {

			def getPaxDetailsList = getPaxTypeList[l].split("-")
			def passengerCriteria_Payload = jsonSlurper.parseText('{"@type": "PassengerCriteria", "number": 1,"passengerTypeCode": "ADT"}')
			passengerCriteria_Payload.number = getPaxDetailsList[0].toInteger()
			passengerCriteria_Payload.passengerTypeCode = getPaxDetailsList[1]
			searchPayloadFormation.CatalogOfferingsQueryRequest.CatalogOfferingsRequest[0].PassengerCriteria[l] = passengerCriteria_Payload
		}

		def jsonSearchPayloadFormation = new JsonOutput().toJson(searchPayloadFormation)
		GlobalVariable.Search_Payload_V9 = jsonSearchPayloadFormation
	}

	@And("the Content Source is (.*) ReturnBrandedFaresInd is (.*)")
	def add_contentsource_returnbrandedfaresind(String contentSource, String returnBrandedFaresInd) {
		def searchPayloadFormation = jsonSlurper.parseText(GlobalVariable.Search_Payload_V9)
		if(contentSource != '') {
			searchPayloadFormation.CatalogOfferingsQueryRequest.CatalogOfferingsRequest[0].contentSourceList.add(contentSource)
		}
		if(returnBrandedFaresInd != '') {
			searchPayloadFormation.CatalogOfferingsQueryRequest.CatalogOfferingsRequest[0].returnBrandedFaresInd = returnBrandedFaresInd.toBoolean()
		}
		def jsonSearchPayloadFormation = new JsonOutput().toJson(searchPayloadFormation)
		GlobalVariable.Search_Payload_V9 = jsonSearchPayloadFormation
	}

	@And("the Carrier Preference Type is (.*) Carrier is (.*)")
	def add_carrierpreferencetype_carrier(String carrierPreferenceType, String carrier) {
		if ((carrierPreferenceType != '') && (carrier != '')) {
			def searchPayloadFormation = jsonSlurper.parseText(GlobalVariable.Search_Payload_V9)
			def carrierPreference_Payload = jsonSlurper.parseText('{ "type": "Permitted","carriers": [ ]}')
			carrierPreference_Payload.type = carrierPreferenceType
			def carrierList = carrier.split(",")
			carrierPreference_Payload.carriers.addAll(carrierList)
			searchPayloadFormation.CatalogOfferingsQueryRequest.CatalogOfferingsRequest[0].SearchModifiersAir.CarrierPreference.add(carrierPreference_Payload)
			def jsonSearchPayloadFormation = new JsonOutput().toJson(searchPayloadFormation)
			GlobalVariable.Search_Payload_V9 = jsonSearchPayloadFormation
		}
	}

	@And("the Cabin Preference Type is (.*) Cabin is (.*)")
	def add_cabinpreferencetype_cabin(String cabinPreferenceType, String cabin) {
		if ((cabinPreferenceType != '') && (cabin != '')) {
			def searchPayloadFormation = jsonSlurper.parseText(GlobalVariable.Search_Payload_V9)
			def cabinPreference_Payload = jsonSlurper.parseText('{ "type": "PreferredWithUpgrade","cabins": [ ] }')
			cabinPreference_Payload.type = cabinPreferenceType
			def cabinList = cabin.split(",")
			cabinPreference_Payload.cabins.addAll(cabinList)
			searchPayloadFormation.CatalogOfferingsQueryRequest.CatalogOfferingsRequest[0].SearchModifiersAir.CabinPreference.add(cabinPreference_Payload)
			def jsonSearchPayloadFormation = new JsonOutput().toJson(searchPayloadFormation)
			GlobalVariable.Search_Payload_V9 = jsonSearchPayloadFormation
		}
	}

	@And("the Currency Code is (.*) ProhibitMaxStayFaresInd is (.*) ProhibitAdvancePurchaseFaresInd is (.*) Fare Type is (.*)")
	def add_currencycode_prohibitmaxstayfaresind_prohibitadvancepurchasefaresind_faretype(String currencyCode, String prohibitMaxStayFaresInd, String prohibitAdvancePurchaseFaresInd, String fareType) {
		def searchPayloadFormation = jsonSlurper.parseText(GlobalVariable.Search_Payload_V9)
		def pricingModifiers_Payload = jsonSlurper.parseText('{"@type": "FareSelectionDetail"}')
		def flag = false
		if(currencyCode != '') {
			searchPayloadFormation.CatalogOfferingsQueryRequest.CatalogOfferingsRequest[0].PricingModifiersAir.currencyCode = currencyCode
		}
		if(fareType != '') {
			pricingModifiers_Payload.fareType = fareType
			searchPayloadFormation.CatalogOfferingsQueryRequest.CatalogOfferingsRequest[0].PricingModifiersAir.FareSelection= pricingModifiers_Payload
		}
		if(prohibitMaxStayFaresInd != '') {
			pricingModifiers_Payload.prohibitMaxStayFaresInd = prohibitMaxStayFaresInd.toBoolean()
			searchPayloadFormation.CatalogOfferingsQueryRequest.CatalogOfferingsRequest[0].PricingModifiersAir.FareSelection= pricingModifiers_Payload
		}
		if(prohibitAdvancePurchaseFaresInd != '') {
			pricingModifiers_Payload.prohibitAdvancePurchaseFaresInd = prohibitAdvancePurchaseFaresInd.toBoolean()
			searchPayloadFormation.CatalogOfferingsQueryRequest.CatalogOfferingsRequest[0].PricingModifiersAir.FareSelection= pricingModifiers_Payload
		}
		def jsonSearchPayloadFormation = new JsonOutput().toJson(searchPayloadFormation)
		GlobalVariable.Search_Payload_V9 = jsonSearchPayloadFormation
	}

	@When("the Search Service with Access Group as (.*) is invoked")
	def invoke_search(String accessGroup) {
		println("invoke SearchService")
		def searchResponse = WS.sendRequestAndVerify(findTestObject('Trip Services/SearchV9', [('AuthToken') : GlobalVariable.token
			, ('AccessGroup') : accessGroup, ('Search_Payload_V9') : GlobalVariable.Search_Payload_V9]))

		WS.verifyResponseStatusCode(searchResponse, 200)

		def jsonSerachRes = jsonSlurper.parseText(searchResponse.getResponseBodyContent())

		GlobalVariable.ShopResponse = jsonSerachRes
	}

	@Then("Validate Search Response")
	def validate_search_response() {
		def jsonSerachRes = GlobalVariable.ShopResponse
		int sequence = calculateSequence()
		generateAndValidateFlightDetails(jsonSerachRes, sequence)
		validatePriceDetails(jsonSerachRes)
	}

	def int calculateSequence() {
		def searchRequestObj = jsonSlurper.parseText(GlobalVariable.Search_Payload_V9)
		int sequence = searchRequestObj.CatalogOfferingsQueryRequest.CatalogOfferingsRequest[0].SearchCriteriaFlight.findAll().size()
		return sequence
	}

	def generateAndValidateFlightDetails(jsonSerachRes, int sequence) {
		List<String> flightRefs = generateFlightRefs(jsonSerachRes,sequence)
		List<String> flightList = generateFlightList(jsonSerachRes, flightRefs)
		validateFlighDetails(flightList)
	}

	def generateFlightRefs(def searchResponse, int sequence) {
		def flightRefs = new ArrayList<>();
		def flightSegmentList = searchResponse.CatalogOfferingsResponse.CatalogOfferings.CatalogOffering.getAt(0).ProductOptions.get(0).Product.FlightSegment.get(0)
		for(def flightSegment : flightSegmentList) {
			flightRefs.add(flightSegment.Flight.FlightRef)
		}
		return flightRefs
	}

	def generateFlightList(def searchResponse, List<String> flightRefs) {
		List<String> flightsList = new ArrayList<String>();
		List<String> flightList = searchResponse.CatalogOfferingsResponse.ReferenceList[0].Flight
		for(eachFlightreference in flightRefs) {
			for(eachFlight in flightList) {
				if(eachFlight.id.equalsIgnoreCase(eachFlightreference)) {
					flightsList.add(eachFlight)
					break;
				}
			}
		}
		return flightsList
	}

	def validateFlighDetails(List flightList) {
		for(eachFlight in flightList) {
			def departureLocation = eachFlight.Departure.location
			GlobalVariable.Origin = departureLocation
			println("departureLocation" + GlobalVariable.Origin)
			def departureDate = eachFlight.Departure.date
			def departureTime = eachFlight.Departure.time
			GlobalVariable.DepartureTime = (departureDate + 'T' + departureTime + '+01:00')
			println("DepartureTime" + GlobalVariable.DepartureTime)
			if(departureLocation!=null && departureDate!=null && departureTime!=null) {
				KeywordUtil.logInfo("DepartureLocation, DepartureDate and DepartureTime are available")
				def arrivalLocation = eachFlight.Arrival.location
				GlobalVariable.Destination = arrivalLocation
				println("arrivalLocation" + GlobalVariable.Destination)
				def arrivalDate = eachFlight.Arrival.date
				def arrivalTime = eachFlight.Arrival.time
				GlobalVariable.ArrivalTime = (arrivalDate + 'T' + arrivalTime + '+01:00')
				println("ArrivalTime" + GlobalVariable.ArrivalTime)
				if(arrivalLocation!=null && arrivalDate!=null && arrivalTime!=null) {
					KeywordUtil.logInfo("ArrivalLocation, ArrivalDate and ArrivalTime are available")
					def carrier = eachFlight.carrier
					GlobalVariable.Carrier = carrier
					println("carrier" + carrier)
					def flightNo = eachFlight.number
					GlobalVariable.FlightNumber = flightNo
					println("flightNo" + flightNo)
					def distance = eachFlight.distance
					GlobalVariable.Distance = distance
					def equipment = eachFlight.equipment
					GlobalVariable.Equipment = equipment

					if(carrier!=null && flightNo!=null) {
						KeywordUtil.logInfo("Carrier and Flightnumber are available")
					}else {
						KeywordUtil.markFailed("Carrier and Flightnumber are not available")
					}
				}else {
					KeywordUtil.markFailed("ArrivalLocation, ArrivalDate and ArrivalTime are not available")
				}
			}else {
				KeywordUtil.markFailed("DepartureLocation, DepartureDate and DepartureTime are not available")
			}
		}
	}

	def validatePriceDetails(def searchResponse) {
		KeywordUtil.logInfo("Shop response price validation is started")
		def totalPrice = searchResponse.CatalogOfferingsResponse.CatalogOfferings.CatalogOffering[0].Price.TotalPrice

		if(totalPrice != null) {
			KeywordUtil.logInfo("Total price is available in Shop Response")
		}else {
			KeywordUtil.markFailed("Total price is not available in Shop Response")
		}
	}

	@And("RoundTrip segments")
	def RoundTrip_segments() {
		def jsonSerachRes = GlobalVariable.ShopResponse
		int sequence = calculate2Sequence()
		generateAndValidate2FlightDetails(jsonSerachRes, sequence)
		validatePriceDetails(jsonSerachRes)
	}

	def int calculate2Sequence() {
		def searchRequestObj = jsonSlurper.parseText(GlobalVariable.Search_Payload_V9)
		int sequence = searchRequestObj.CatalogOfferingsQueryRequest.CatalogOfferingsRequest[0].SearchCriteriaFlight.findAll().size()
		println("sequence" + sequence)
		return sequence
	}
	def generateAndValidate2FlightDetails(jsonSerachRes, int sequence) {
		String flight2Refs = generate2flight2Refs(jsonSerachRes,sequence)
		List<String> flight2List = generate2flight2List(jsonSerachRes, flight2Refs)
		validate2FlighDetails(flight2List)
	}


	def generate2flight2Refs(def searchResponse, int sequence) {

		String flight2Refs = searchResponse.CatalogOfferingsResponse.CatalogOfferings.CatalogOffering[0].ProductOptions[1].Product[0].FlightSegment[0].Flight.FlightRef
		println("flight2Refs" + flight2Refs)
		return flight2Refs
	}

	def generate2flight2List(def searchResponse, String flight2Refs) {
		List<String> flights2List = new ArrayList<String>();
		List<String> flight2List = searchResponse.CatalogOfferingsResponse.ReferenceList[0].Flight
		for(each2Flight in flight2List) {
			if(each2Flight.id.equalsIgnoreCase(flight2Refs)) {
				flights2List.add(each2Flight)
				break;
			}
		}

		return flights2List
	}

	def validate2FlighDetails(List flight2List) {
		for(each2Flight in flight2List) {
			def departureLocation2 = each2Flight.Departure.location
			GlobalVariable.Origin2 = departureLocation2
			println("departureLocation2" + GlobalVariable.Origin2)
			def departureDate2 = each2Flight.Departure.date
			def departureTime2 = each2Flight.Departure.time
			GlobalVariable.DepartureTime2 = (departureDate2 + 'T' + departureTime2 + '+01:00')
			println("DepartureTime2" + GlobalVariable.DepartureTime2)
			if(departureLocation2!=null && departureDate2!=null && departureTime2!=null) {
				KeywordUtil.logInfo("DepartureLocation, DepartureDate and DepartureTime are available")
				def arrivalLocation2 = each2Flight.Arrival.location
				GlobalVariable.Destination2 = arrivalLocation2
				println("arrivalLocation2" + GlobalVariable.Destination2)
				def arrivalDate2 = each2Flight.Arrival.date
				def arrivalTime2 = each2Flight.Arrival.time
				GlobalVariable.ArrivalTime2 = (arrivalDate2 + 'T' + arrivalTime2 + '+01:00')
				println("ArrivalTime2" + GlobalVariable.ArrivalTime2)
				if(arrivalLocation2!=null && arrivalDate2!=null && arrivalTime2!=null) {
					KeywordUtil.logInfo("ArrivalLocation, ArrivalDate and ArrivalTime are available")
					def carrier2 = each2Flight.carrier
					GlobalVariable.Carrier2 = carrier2
					println("carrier2" + carrier2)
					def flightNo2 = each2Flight.number
					GlobalVariable.FlightNumber2 = flightNo2
					println("flightNo2" + flightNo2)
					def distance2 = each2Flight.distance
					GlobalVariable.Distance2 = distance2
					def equipment2 = each2Flight.equipment
					GlobalVariable.Equipment2 = equipment2
					if(carrier2!=null && flightNo2!=null) {
						KeywordUtil.logInfo("Carrier and Flightnumber are available")
					}else {
						KeywordUtil.markFailed("Carrier and Flightnumber are not available")
					}
				}else {
					KeywordUtil.markFailed("ArrivalLocation, ArrivalDate and ArrivalTime are not available")
				}
			}else {
				KeywordUtil.markFailed("DepartureLocation, DepartureDate and DepartureTime are not available")
			}
		}
	}
}