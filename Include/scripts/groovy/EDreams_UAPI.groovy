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



class EDreams_UAPI {
	/**
	 * The step definitions below match with Katalon sample Gherkin steps
	 */
	static def PriceRsp
	static def AirBookRsp
	static def URRetrieveRsp
	static def UR
	static def Response2
	def slurper = new groovy.json.JsonSlurper()
	def jsonSlurper = new JsonSlurper()

	@Given("UAPI Price transaction for 1G provider")
	def UAPI_Price_transaction_for_1G_provider() {
		println "UAPI Price transaction for 1G provider"
	}

	@When("AirItinerary has air segments from tripservice SearchResponse")
	def AirItinerary_has_air_segments_from_tripservice_SearchResponse() {
		println "AirItinerary has air segments from tripservice SearchResponse"
	}

	@Then("Price Transaction should be successful")
	def Price_Transaction_should_be_successful() {
		println "Price Transaction should be successful"
		PriceRsp = WS.sendRequest(findTestObject('PRPEdreams_31RB_TC01/AirPrice_EDreams_E2E', [('Carrier') : GlobalVariable.Carrier
			, ('FlightNumber') : GlobalVariable.FlightNumber, ('Origin') : GlobalVariable.Origin, ('Destination') : GlobalVariable.Destination
			, ('DepartureTime') : GlobalVariable.DepartureTime, ('ArrivalTime') : GlobalVariable.ArrivalTime, ('Distance') : GlobalVariable.Distance
			, ('Equipment') : GlobalVariable.Equipment]))

		String key = WS.getElementText(PriceRsp, 'AirPriceRsp.AirPriceResult.AirPricingSolution[0].AirSegmentRef[0].@Key')

		WS.verifyElementText(PriceRsp, 'AirPriceRsp.AirItinerary.AirSegment[0].@Key', key)

		WS.verifyResponseStatusCode(PriceRsp, 200)
	}

	@Given("uapi AirCreateReservation transaction")
	def uapi_AirCreateReservation_transaction() {
		println "uapi AirCreateReservation transaction"
	}


	@When("AirCreateReservationReq has BookingTraveler and FormOfPayment details")
	def AirCreateReservationReq_has_BookingTraveler_and_FormOfPayment_details() {
		println "AirCreateReservationReq has BookingTraveler and FormOfPayment details"
	}

	@And("add PricingSolution from the pricing Response")
	def add_PricingSolution_from_the_pricing_Response() {
		println "add PricingSolution from the pricing Response"

		String xml2 = PriceRsp.getResponseBodyContent()
		def APS = CustomKeywords.'uapiFunctions.Libraries.getPricingSolutions'(xml2)
		GlobalVariable.APS = APS

		CustomKeywords.'uapiFunctions.Libraries.addBasicAuthorizationProperty'(findTestObject('PRPEdreams_31RB_TC01/AirCreateReservation',
				[('APS') : GlobalVariable.APS, ('TargetBranch') : GlobalVariable.TargetBranch, ('Carrier') : GlobalVariable.Carrier]),
				GlobalVariable.Username, GlobalVariable.Password)
	}


	@Then("AirCreateReservation should be successful")
	def AirCreateReservation_should_be_successful() {
		println "AirCreateReservation should be successful"

		AirBookRsp = WS.sendRequest(findTestObject('PRPEdreams_31RB_TC01/AirCreateReservation', [('APS') : GlobalVariable.APS, ('TargetBranch') : GlobalVariable.TargetBranch
			, ('Carrier') : GlobalVariable.Carrier]))

		WS.verifyResponseStatusCode(AirBookRsp, 200)
	}

	@Given("uapi UniversalRecordRetrieve transaction")
	def uapi_UniversalRecordRetrieve_transaction() {
		println "uapi UniversalRecordRetrieve transaction"
	}


	@When("request has UniversalRecordlocatorCode from the AirCreateReservation Response")
	def request_has_UniversalRecordlocatorCode_from_the_AirCreateReservation_Response() {
		println "request has UniversalRecordlocatorCode from the AirCreateReservation Response"
		UR = WS.getElementText(AirBookRsp, 'AirCreateReservationRsp.UniversalRecord.@LocatorCode')

		def PNR = WS.getElementText(AirBookRsp, 'AirCreateReservationRsp.UniversalRecord.ProviderReservationInfo.@LocatorCode')

		GlobalVariable.UR = UR
		println("GlobalVariable.UR" + GlobalVariable.UR)

		GlobalVariable.PNR = PNR
		println("GlobalVariable.PNR" + GlobalVariable.PNR)

		CustomKeywords.'uapiFunctions.Libraries.addBasicAuthorizationProperty'(findTestObject('PRPEdreams_31RB_TC01/URRetrieve',
				[('APS') : GlobalVariable.APS, ('TargetBranch') : GlobalVariable.TargetBranch, ('UR') : GlobalVariable.UR]), GlobalVariable.Username,
				GlobalVariable.Password)
	}



	@Then("UniversalRecordRetrieve should be successful")
	def UniversalRecordRetrieve_should_be_successful() {
		println "UniversalRecordRetrieve should be successful"
		URRetrieveRsp = WS.sendRequest(findTestObject('PRPEdreams_31RB_TC01/URRetrieve', [('APS') : GlobalVariable.APS, ('TargetBranch') : GlobalVariable.TargetBranch
			, ('UR') : GlobalVariable.UR]))

		String xml3 = URRetrieveRsp.responseBodyContent

		WS.verifyResponseStatusCode(URRetrieveRsp, 200)

		def UR1 = WS.getElementText(URRetrieveRsp, 'UniversalRecordRetrieveRsp.UniversalRecord.@LocatorCode')

		WS.verifyElementText(URRetrieveRsp, 'UniversalRecordRetrieveRsp.UniversalRecord.@LocatorCode', UR)
	}

	@Given("uapi Ticketing transaction")
	def uapi_Ticketing_transaction() {
		println "uapi Ticketing transaction"
	}


	@When("request has AiReservationLocatorCode from the UniversalRecordRetrieve Response")
	def request_has_AiReservationLocatorCode_from_the_UniversalRecordRetrieve_Response() {
		println "request has AiReservationLocatorCode from the UniversalRecordRetrieve Response"

		String recLoc = WS.getElementText(URRetrieveRsp, 'UniversalRecordRetrieveRsp.UniversalRecord.AirReservation.@LocatorCode')

		String carrierLoc = WS.getElementText(URRetrieveRsp, 'UniversalRecordRetrieveRsp.UniversalRecord.AirReservation.SupplierLocator.@SupplierLocatorCode')

		GlobalVariable.RecordLocator = recLoc
		println("GlobalVariable.RecordLocator" + GlobalVariable.RecordLocator)

		GlobalVariable.CLC = carrierLoc
		println("GlobalVariable.CLC" + GlobalVariable.CLC)

		CustomKeywords.'uapiFunctions.Libraries.addBasicAuthorizationProperty'(findTestObject('PRPEdreams_31RB_TC01/Ticketing',
				[('AirRecordLocator') : GlobalVariable.RecordLocator]), GlobalVariable.Username, GlobalVariable.Password)
		recLoc = null
		carrierLoc = null
	}



	@Then("Ticketing should be successful")
	def Ticketing_should_be_successful() {
		println "Ticketing should be successful"
		def TicketingRsp = WS.sendRequestAndVerify(findTestObject('PRPEdreams_31RB_TC01/Ticketing', [('AirRecordLocator') : GlobalVariable.RecordLocator]))
	}

	@Given("UAPI AirPrice transaction for 1G providercode")
	def UAPI_AirPrice_transaction_for_1G_providecoder() {
		println "UAPI AirPrice transaction for 1G providercode"
	}


	@When("AirItinerary has segments from tripservice SearchResponse")
	def AirItinerary_has_segments_from_tripservice_SearchResponse() {
		println "AirItinerary has segments from tripservice SearchResponse"
	}


	@Then("AirPrice Transaction should be successful")
	def AirPrice_Transaction_should_be_successful() {
		println "AirPrice Transaction should be successful"

		PriceRsp = WS.sendRequest(findTestObject('PRPEdreams_31RB_TC02/AirPrice_EDreams_E2E', [('Carrier') : GlobalVariable.Carrier
			, ('FlightNumber') : GlobalVariable.FlightNumber, ('Origin') : GlobalVariable.Origin, ('Destination') : GlobalVariable.Destination
			, ('DepartureTime') : GlobalVariable.DepartureTime, ('ArrivalTime') : GlobalVariable.ArrivalTime, ('Distance') : GlobalVariable.Distance
			, ('Equipment') : GlobalVariable.Equipment, ('Carrier2') : GlobalVariable.Carrier2, ('FlightNumber2') : GlobalVariable.FlightNumber2
			, ('Origin2') : GlobalVariable.Origin2, ('Destination2') : GlobalVariable.Destination2, ('DepartureTime2') : GlobalVariable.DepartureTime2, ('ArrivalTime2') : GlobalVariable.ArrivalTime2, ('Distance2') : GlobalVariable.Distance2, ('Equipment2') : GlobalVariable.Equipment2]))

		String key = WS.getElementText(PriceRsp, 'AirPriceRsp.AirPriceResult.AirPricingSolution[0].AirSegmentRef[0].@Key')

		WS.verifyElementText(PriceRsp, 'AirPriceRsp.AirItinerary.AirSegment[0].@Key', key)

		WS.verifyResponseStatusCode(PriceRsp, 200)
	}

	@Given("uapi AirBook transaction")
	def uapi_AirBook_transaction() {
		println "uapi AirBook transaction"
	}


	@When("AirBook Request has BookingTraveler and FormOfPayment details")
	def AirBook_Request_has_BookingTraveler_and_FormOfPayment_details() {
		println "AirBook Request has BookingTraveler and FormOfPayment details"
	}

	@And("add AirPricingSolution from the airpricing Response")
	def add_AirPricingSolution_from_the_airpricing_Response() {
		println "add AirPricingSolution from the airpricing Response"

		String xml2 = PriceRsp.getResponseBodyContent()

		def APS = CustomKeywords.'uapiFunctions.Libraries.getPricingSolutions'(xml2)

		GlobalVariable.APS = APS

		CustomKeywords.'uapiFunctions.Libraries.addBasicAuthorizationProperty'(findTestObject('PRPEdreams_31RB_TC02/AirCreateReservation',
				[('APS') : GlobalVariable.APS, ('TargetBranch') : GlobalVariable.TargetBranch, ('Carrier') : GlobalVariable.Carrier]),
				GlobalVariable.Username, GlobalVariable.Password)
	}


	@Then("AirBook should be successful")
	def AirBook_should_be_successful() {
		println "AirBook should be successful"
		AirBookRsp = WS.sendRequest(findTestObject('PRPEdreams_31RB_TC02/AirCreateReservation', [('APS') : GlobalVariable.APS, ('TargetBranch') : GlobalVariable.TargetBranch
			, ('Carrier') : GlobalVariable.Carrier]))

		WS.verifyResponseStatusCode(AirBookRsp, 200)
	}

	@Given("uapi URRetrieve transaction")
	def uapi_URRetrieve_transaction() {
		println "uapi URRetrieve transaction"
	}


	@When("request has UniversalRecordlocatorCode from the AirBook Response")
	def request_has_UniversalRecordlocatorCode_from_the_AirBook_Response() {
		println "request has UniversalRecordlocatorCode from the AirBook Response"

		UR = WS.getElementText(AirBookRsp, 'AirCreateReservationRsp.UniversalRecord.@LocatorCode')

		def PNR = WS.getElementText(AirBookRsp, 'AirCreateReservationRsp.UniversalRecord.ProviderReservationInfo.@LocatorCode')

		GlobalVariable.UR = UR
		println("UR" + GlobalVariable.UR)

		GlobalVariable.PNR = PNR
		println("PNR" + GlobalVariable.PNR)

		CustomKeywords.'uapiFunctions.Libraries.addBasicAuthorizationProperty'(findTestObject('PRPEdreams_31RB_TC02/URRetrieve',
				[('APS') : GlobalVariable.APS, ('TargetBranch') : GlobalVariable.TargetBranch, ('UR') : GlobalVariable.UR]), GlobalVariable.Username,
				GlobalVariable.Password)
	}

	@Then("URRetrieve should be successful")
	def URRetrieve_should_be_successful() {
		println "URRetrieve should be successful"

		URRetrieveRsp = WS.sendRequest(findTestObject('PRPEdreams_31RB_TC02/URRetrieve', [('APS') : GlobalVariable.APS, ('TargetBranch') : GlobalVariable.TargetBranch
			, ('UR') : GlobalVariable.UR]))

		String xml3 = URRetrieveRsp.responseBodyContent

		WS.verifyResponseStatusCode(URRetrieveRsp, 200)

		def UR1 = WS.getElementText(URRetrieveRsp, 'UniversalRecordRetrieveRsp.UniversalRecord.@LocatorCode')

		WS.verifyElementText(URRetrieveRsp, 'UniversalRecordRetrieveRsp.UniversalRecord.@LocatorCode', UR)
	}

	@Given("uapi AirTicketing transaction")
	def uapi_AirTicketing_transaction() {
		println "uapi AirTicketing transaction"
	}

	@When("request has AiReservationLocatorCode from the URRetrieve Response")
	def request_has_AiReservationLocatorCode_from_the_URRetrieve_Response() {

		println "request has AiReservationLocatorCode from the URRetrieve Response"

		GlobalVariable.RecordLocator = WS.getElementText(URRetrieveRsp, 'UniversalRecordRetrieveRsp.UniversalRecord.AirReservation.@LocatorCode')

		println("RecordLocator" + GlobalVariable.RecordLocator)

		GlobalVariable.CLC = WS.getElementText(URRetrieveRsp, 'UniversalRecordRetrieveRsp.UniversalRecord.AirReservation.SupplierLocator.@SupplierLocatorCode')
		println("CLC" + GlobalVariable.CLC)

		CustomKeywords.'uapiFunctions.Libraries.addBasicAuthorizationProperty'(findTestObject('PRPEdreams_31RB_TC02/Ticketing',
				[('AirRecordLocator') : GlobalVariable.RecordLocator]), GlobalVariable.Username, GlobalVariable.Password)
	}

	@When("request has AiReservationLocatorCode from the UniversalRecordRetrieveResponse")
	def request_has_AiReservationLocatorCode_from_the_UniversalRecordRetrieveResponse() {

		println "request has AiReservationLocatorCode from the UniversalRecordRetrieveResponse"

		String recLoc = WS.getElementText(URRetrieveRsp, 'UniversalRecordRetrieveRsp.UniversalRecord.AirReservation.@LocatorCode')

		GlobalVariable.RecordLocator = recLoc
		println("RecordLocator" + GlobalVariable.RecordLocator)

		String carrierLoc = WS.getElementText(URRetrieveRsp, 'UniversalRecordRetrieveRsp.UniversalRecord.AirReservation.SupplierLocator.@SupplierLocatorCode')
		GlobalVariable.CLC = carrierLoc
		println("CLC" + GlobalVariable.CLC)


		CustomKeywords.'uapiFunctions.Libraries.addBasicAuthorizationProperty'(findTestObject('PRPEdreams_31RB_TC02/Ticketing',
				[('AirRecordLocator') : GlobalVariable.RecordLocator]), GlobalVariable.Username, GlobalVariable.Password)

		recLoc = null
		carrierLoc = null
	}

	@Then("AirTicketing should be successful")
	def AirTicketing_should_be_successful() {
		println "AirTicketing should be successful"

		def TicketingRsp = WS.sendRequestAndVerify(findTestObject('PRPEdreams_31RB_TC02/Ticketing', [('AirRecordLocator') : GlobalVariable.RecordLocator]))
	}
	@Given("UAPI Pricing transaction for 1G providercode")
	def UAPI_Pricing_transaction_for_1G_providecoder() {
		println "UAPI Pricing transaction for 1G providercode"
	}


	@When("AirItinerary has airsegments from tripservice SearchResponse")
	def AirItinerary_has_airsegments_from_tripservice_SearchResponse() {
		println "AirItinerary has airsegments from tripservice SearchResponse"
	}


	@Then("Pricing Transaction should be successfull")
	def Pricing_Transaction_should_be_successfull() {
		println "Pricing Transaction should be successfull"
		PriceRsp = WS.sendRequest(findTestObject('PRPEdreams_31RB_TC03/AirPrice_EDreams_E2E', [('Carrier') : GlobalVariable.Carrier
			, ('FlightNumber') : GlobalVariable.FlightNumber, ('Origin') : GlobalVariable.Origin, ('Destination') : GlobalVariable.Destination
			, ('DepartureTime') : GlobalVariable.DepartureTime, ('ArrivalTime') : GlobalVariable.ArrivalTime, ('Distance') : GlobalVariable.Distance
			, ('Equipment') : GlobalVariable.Equipment, ('Carrier2') : GlobalVariable.Carrier2, ('FlightNumber2') : GlobalVariable.FlightNumber2
			, ('Origin2') : GlobalVariable.Origin2, ('Destination2') : GlobalVariable.Destination2, ('DepartureTime2') : GlobalVariable.DepartureTime2
			, ('ArrivalTime2') : GlobalVariable.ArrivalTime2, ('Distance2') : GlobalVariable.Distance2, ('Equipment2') : GlobalVariable.Equipment2]))

		String key = WS.getElementText(PriceRsp, 'AirPriceRsp.AirPriceResult.AirPricingSolution[0].AirSegmentRef[0].@Key')

		WS.verifyElementText(PriceRsp, 'AirPriceRsp.AirItinerary.AirSegment[0].@Key', key)

		WS.verifyResponseStatusCode(PriceRsp, 200)
	}

	@Given("uapi Booking transaction")
	def uapi_Booking_transaction() {
		println "uapi Booking transaction"
	}


	@When("AirCreateReservationRequest has multi BookingTraveler and FormOfPayment details")
	def AirCreateReservationRequest_has_multi_BookingTraveler_and_FormOfPayment_details() {
		println "AirCreateReservationRequest has multi BookingTraveler and FormOfPayment details"
	}

	@And("add AirPricingSolution from the pricing Response")
	def add_AirPricingSolution_from_the_pricing_Response() {
		println "add AirPricingSolution from the pricing Response"

		String xml2 = PriceRsp.getResponseBodyContent()
		def APS = CustomKeywords.'uapiFunctions.Libraries.getPricingSolutions'(xml2)
		GlobalVariable.APS = APS

		CustomKeywords.'uapiFunctions.Libraries.addBasicAuthorizationProperty'(findTestObject('PRPEdreams_31RB_TC03/AirCreateReservation',
				[('APS') : GlobalVariable.APS, ('TargetBranch') : GlobalVariable.TargetBranch, ('Carrier') : GlobalVariable.Carrier]),
				GlobalVariable.Username, GlobalVariable.Password)
	}


	@Then("AirCreateReservationRequest transaction should be successful")
	def AirCreateReservationRequest_transaction_should_be_successful() {
		println "AirCreateReservationRequest transaction should be successful"
		AirBookRsp = WS.sendRequest(findTestObject('PRPEdreams_31RB_TC03/AirCreateReservation', [('APS') : GlobalVariable.APS, ('TargetBranch') : GlobalVariable.TargetBranch
			, ('Carrier') : GlobalVariable.Carrier]))

		WS.verifyResponseStatusCode(AirBookRsp, 200)
	}

	@Given("uapi UniversalRetrieve transaction")
	def uapi_UniversalRetrieve_transaction() {
		println "uapi UniversalRetrieve transaction"
	}

	@When("request has UniversalRecordlocatorCode from the AirCreate Response")
	def request_has_UniversalRecordlocatorCode_from_the_AirCreate_Response() {
		println "request has UniversalRecordlocatorCode from the AirCreate Response"
		UR = WS.getElementText(AirBookRsp, 'AirCreateReservationRsp.UniversalRecord.@LocatorCode')

		def PNR = WS.getElementText(AirBookRsp, 'AirCreateReservationRsp.UniversalRecord.ProviderReservationInfo.@LocatorCode')

		GlobalVariable.UR = UR

		GlobalVariable.PNR = PNR

		CustomKeywords.'uapiFunctions.Libraries.addBasicAuthorizationProperty'(findTestObject('PRPEdreams_31RB_TC03/URRetrieve',
				[('APS') : GlobalVariable.APS, ('TargetBranch') : GlobalVariable.TargetBranch, ('UR') : GlobalVariable.UR]), GlobalVariable.Username,
				GlobalVariable.Password)
	}

	@Then("UniversalRecordRetrieve transaction should be successfull")
	def UniversalRecordRetrieve_transaction_should_be_successfull() {
		println "UniversalRecordRetrieve transaction should be successfull"
		URRetrieveRsp = WS.sendRequest(findTestObject('PRPEdreams_31RB_TC03/URRetrieve', [('APS') : GlobalVariable.APS, ('TargetBranch') : GlobalVariable.TargetBranch
			, ('UR') : GlobalVariable.UR]))

		String xml3 = URRetrieveRsp.responseBodyContent

		WS.verifyResponseStatusCode(URRetrieveRsp, 200)

		def UR1 = WS.getElementText(URRetrieveRsp, 'UniversalRecordRetrieveRsp.UniversalRecord.@LocatorCode')

		WS.verifyElementText(URRetrieveRsp, 'UniversalRecordRetrieveRsp.UniversalRecord.@LocatorCode', UR)
	}

	@Given("uapi Ticket transaction")
	def uapi_Ticket_transaction() {
		println "uapi Ticket transaction"
	}


	@When("request has the AirReservationLocatorCode from the UniversalRecordRetrieve Response")
	def request_has_the_AirReservationLocatorCode_from_the_UniversalRecordRetrieve_Response() {
		println "request has the AirReservationLocatorCode from the UniversalRecordRetrieve Response"

		String recLoc = WS.getElementText(URRetrieveRsp, 'UniversalRecordRetrieveRsp.UniversalRecord.AirReservation.@LocatorCode')

		String carrierLoc = WS.getElementText(URRetrieveRsp, 'UniversalRecordRetrieveRsp.UniversalRecord.AirReservation.SupplierLocator.@SupplierLocatorCode')

		GlobalVariable.RecordLocator = recLoc

		GlobalVariable.CLC = carrierLoc

		CustomKeywords.'uapiFunctions.Libraries.addBasicAuthorizationProperty'(findTestObject('PRPEdreams_31RB_TC03/Ticketing',
				[('AirRecordLocator') : GlobalVariable.RecordLocator]), GlobalVariable.Username, GlobalVariable.Password)

		recLoc = null
		carrierLoc = null
	}

	@Then("Ticketing transaction should be successfull")
	def Ticketing_transaction_should_be_successfull() {
		println "Ticketing transaction should be successfull"
		def TicketingRsp = WS.sendRequestAndVerify(findTestObject('PRPEdreams_31RB_TC03/Ticketing', [('AirRecordLocator') : GlobalVariable.RecordLocator]))
	}

	@When("request has UniversalRecordlocatorCode from the UniversalRecordRetrieve Response")
	def request_has_UniversalRecordlocatorCode_from_the_UniversalRecordRetrieve_Response() {
		println "request has UniversalRecordlocatorCode from the UniversalRecordRetrieve Response"
	}

	@And("should have Ticketing details")
	def should_have_Ticketing_details() {
		println "should have Ticketing details"

		CustomKeywords.'uapiFunctions.Libraries.addBasicAuthorizationProperty'(findTestObject('PRPEdreams_31RB_TC03/AMOA_1', [('AirRecordLocator') : GlobalVariable.RecordLocator]),
		GlobalVariable.Username, GlobalVariable.Password)
	}

	@Given("uapi AirMerchandisingOfferAvailability transaction")
	def uapi_AirMerchandisingOfferAvailability_transaction() {
		println "uapi AirMerchandisingOfferAvailability transaction"
	}

	@When("request has ProviderLocatorCode, CarrierLocatorCode and Carrier from the UniversalRecordRetrieve Response")
	def request_has_ProviderLocatorCode_CarrierLocatorCode_and_Carrier_from_the_UniversalRecordRetrieve_Response() {
		println "request has ProviderLocatorCode, CarrierLocatorCode and Carrier from the UniversalRecordRetrieve Response"
	}

	@Then("AirMerchandisingOfferAvailability should be successful")
	def AirMerchandisingOfferAvailability_should_be_successful() {
		println "AirMerchandisingOfferAvailability should be successful"
		def AMOARsp = WS.sendRequestAndVerify(findTestObject('PRPEdreams_31RB_TC03/AMOA_1', [('AirRecordLocator') : GlobalVariable.RecordLocator]))

		String xml4 = AMOARsp.responseBodyContent
		def OPS = CustomKeywords.'uapiFunctions.Libraries.GetOptionalServices'(xml4)

		GlobalVariable.OPS = OPS

		def SegAMOA = CustomKeywords.'uapiFunctions.Libraries.GetAirSegmentMerchOffer'(xml4)
		GlobalVariable.SegAMF = SegAMOA

		println(SegAMOA)

		CustomKeywords.'uapiFunctions.Libraries.addBasicAuthorizationProperty'(findTestObject('PRPEdreams_31RB_TC03/AMF', [('AirRecordLocator') : GlobalVariable.RecordLocator
			, ('SegAMF') : GlobalVariable.SegAMF]), GlobalVariable.Username, GlobalVariable.Password)
	}

	@Given("uapi AirMerchandisingFulfillment transaction")
	def uapi_AirMerchandisingFulfillment_transaction() {
		println "uapi AirMerchandisingFulfillment transaction"
	}

	@And("add SearchTraveler, segment, OptionalServices from AirMerchandisingOfferAvailability response")
	def add_SearchTraveler_segment_OptionalServices_from_AirMerchandisingOfferAvailability_response() {
		println "add SearchTraveler, segment, OptionalServices from AirMerchandisingOfferAvailability response"
	}

	@Then("AirMerchandisingFulfillment should be successful")
	def AirMerchandisingFulfillment_should_be_successful() {
		println "AirMerchandisingFulfillment should be successful"
		def AMFRsp = WS.sendRequestAndVerify(findTestObject('PRPEdreams_31RB_TC03/AMF', [('AirRecordLocator') : GlobalVariable.RecordLocator
			, ('SegAMF') : GlobalVariable.SegAMF]))
	}

	@Given("uapi SeatMap transaction")
	def uapi_SeatMap_transaction() {
		println "uapi SeatMap transaction"

		CustomKeywords.'uapiFunctions.Libraries.addBasicAuthorizationProperty'(findTestObject('PRPEdreams_31RB_TC03/SeatMap', [('AirRecordLocator') : GlobalVariable.RecordLocator
			, ('Segment') : GlobalVariable.seg]), GlobalVariable.Username, GlobalVariable.Password)
	}

	@And("SeatMap should be successful")
	def SeatMap_should_be_successful() {
		println "SeatMap should be successful"

		def SeatMapRsp = WS.sendRequestAndVerify(findTestObject('PRPEdreams_31RB_TC03/SeatMap', [('AirRecordLocator') : GlobalVariable.RecordLocator
			, ('Segment') : GlobalVariable.seg]))

		String xml5 = SeatMapRsp.responseBodyContent

		def SeatOPS = CustomKeywords.'uapiFunctions.Libraries.GetOptionalServicesWithSeatMap'(xml5)

		println(SeatOPS)

		GlobalVariable.OPS = SeatOPS

		def SegAMOA1 = CustomKeywords.'uapiFunctions.Libraries.GetAirSegmentMerchOffer'(xml5)

		GlobalVariable.SegAMF = SegAMOA1

		CustomKeywords.'uapiFunctions.Libraries.addBasicAuthorizationProperty'(findTestObject('PRPEdreams_31RB_TC03/AMF', [('AirRecordLocator') : GlobalVariable.RecordLocator
			, ('SegAMF') : GlobalVariable.SegAMF]), GlobalVariable.Username, GlobalVariable.Password)
	}
}