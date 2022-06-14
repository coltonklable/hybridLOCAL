package hybrid
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI


import internal.GlobalVariable

import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.By

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObjectProperty

import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil

import com.kms.katalon.core.webui.exception.WebElementNotFoundException

import cucumber.api.java.en.And
import cucumber.api.java.en.Given
import cucumber.api.java.en.Then
import cucumber.api.java.en.When
import cAPI.Extbase
import cAPI.BDD_HIS




class HIS {
	/**
	 * The step definitions below match with Katalon sample Gherkin steps
	 */
	@Given ("Set manual proxy and Dates")
	def Set_manual_proxy_and_Dates () {
		cAPI.Extbase.setManualProxy()
		cAPI.BDD_HIS.SetDates()

		println ("Set manual proxy and Dates")
	}
	@Then("manual proxy and dates should be set")
	def manual_proxy_and_dates_should_be_set() {
		println ("manual proxy and dates should be set");
	}



	@Given ("UAPI LFS transaction for 1G provider")
	def UAPI_LFS_transaction_for_1G_provider() {

		println ("UAPI LFS transaction for 1G provider")
	}

	@When("the itinerary has oneway segment")
	def the_itinerary_has_oneway_segment() {
		//cAPI.BDD_HIS.SetGlobalVariables("abc", "abc")

		println "the itinerary has oneway segment"
	}

	@And("add LI2 PCC credentials")
	def add_LI2_PCC_credentials() {
		//cAPI.BDD_HIS.SetGlobalVariables("abc", "abc")
		println "add LI2 PCC credentials"
	}

	@Then("LFS Transaction should be successful")
	def LFS_Transaction_should_be_successful() {

		//GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue1)

		//GlobalVariable.DepartureDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyy-MM-dd', GlobalVariable.DateValue1)

		def Response1
		Response1 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/Air LFS', [('TargetBranch') : GlobalVariable.TargetBranch
			, ('Provider') : GlobalVariable.Provider, ('Origin') : GlobalVariable.Origin, ('Destination') : GlobalVariable.Destination
			, ('Carrier') : GlobalVariable.Carrier, ('DepartureDate') : GlobalVariable.DepartureDate]))
		println Response1
		println "abc"
		String xml1 = Response1.responseText

		def result1 = new XmlSlurper().parseText(xml1)

		println('result1' + result1)

		GlobalVariable.Carrier = result1.Body.LowFareSearchRsp.AirSegmentList.AirSegment[0].@Carrier

		println('Carrier' + GlobalVariable.Carrier)

		String FlightNumber = result1.Body.LowFareSearchRsp.AirSegmentList.AirSegment[0].@FlightNumber

		if (FlightNumber.length() < 4) {
			FlightNumber = ('0' + FlightNumber)
		}

		GlobalVariable.FlightNumber = FlightNumber

		GlobalVariable.Origin = result1.Body.LowFareSearchRsp.AirSegmentList.AirSegment[0].@Origin

		GlobalVariable.Destination = result1.Body.LowFareSearchRsp.AirSegmentList.AirSegment[0].@Destination
	}

	@Given ("GWS BeginSession transaction for 1G provider")
	def GWS_BeginSession_transaction_for_1G_provider() {

		println ("GWS BeginSession transaction for 1G provider")
	}

	@When("BeginSession has 8Q8 Profile details")
	def BeginSession_has_8Q8_Profile_detailst() {
		//cAPI.BDD_HIS.SetGlobalVariables("abc", "abc")
		println "BeginSession has 8Q8 Profile details"
	}



	@Then("BeginSession Transaction should be successful")
	def BeginSession_Transaction_should_be_successful() {

		def Response2
		Response2 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/BeginSession', [('Profile') : GlobalVariable.Profile1]))
		println Response2
		println "abc123"
		String xml2 = Response2.responseBodyContent

		def result2 = new XmlSlurper().parseText(xml2)

		GlobalVariable.TokenNumber = result2.BeginSessionResult.text()

		println('TokenNumber' + GlobalVariable.TokenNumber)
	}

	@Given ("GWS PNRBFManagement_49 transaction for booking air segments")
	def GWS_PNRBFManagement_49_transaction_for_booking_air_segments() {

		println ("GWS PNRBFManagement_49 transaction for booking air segments")
	}

	@When("PNRBFManagement_49 Transaction has carrier,FlightNumber,Origin,destination details from UAPI LowFareSearch Response")
	def PNRBFManagement_49_Transaction_has_carrier_FlightNumber_Origin_destination_details_from_UAPI_LowFareSearch_Response() {
		//cAPI.BDD_HIS.SetGlobalVariables("abc", "abc")
		println "PNRBFManagement_49 Transaction has carrier,FlightNumber,Origin,destination details from UAPI LowFareSearch Response"
	}



	@Then("PNRBFManagement_49 Transaction should be successful")
	def PNRBFManagement_49_Transaction_should_be_successful() {

		//GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue1)

		//GlobalVariable.DepartureDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyy-MM-dd', GlobalVariable.DateValue1)

		def Response3
		Response3 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/PNRBFManagement_Create PNR',
				[('TokenNumber') : GlobalVariable.TokenNumber, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider
					, ('Carrier') : GlobalVariable.Carrier, ('FlightNumber') : GlobalVariable.FlightNumber, ('Origin') : GlobalVariable.Origin
					, ('Destination') : GlobalVariable.Destination, ('StartDate') : GlobalVariable.StartDate, ('TicketDate') : GlobalVariable.TicketDate]))

		println Response3
		String xml3 = Response3.responseBodyContent

		def result3 = new XmlSlurper().parseText(xml3)

		GlobalVariable.RecordLoc = result3.SubmitXmlOnSessionResult.PNRBFManagement_49.EndTransaction.EndTransactResponse[0].RecLoc.text()

		println('RecordLoc' + GlobalVariable.RecordLoc)
	}

	@Given ("GWS PNRBFManagement_49 transaction for PNRBFRetrieveMods")
	def GWS_PNRBFManagement_49_transaction_for_PNRBFRetrieveMods() {

		println ("GWS PNRBFManagement_49 transaction for PNRBFRetrieveMods")
	}

	@When("PNRBFManagement_49 Transaction has RecLoc from previous PNRBFManagement_49 Response")
	def PNRBFManagement_49_Transaction_has_RecLoc_from_previous_PNRBFManagement_49_Response() {
		//cAPI.BDD_HIS.SetGlobalVariables("abc", "abc")
		println "PNRBFManagement_49 Transaction has RecLoc from previous PNRBFManagement_49 Response"
	}



	@Then("PNRBFManagement_49 Transaction should be successful for PNRBFRetrieveMods")
	def PNRBFManagement_49_Transaction_should_be_successful_for_PNRBFRetrieveMods() {

		//GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue1)

		//GlobalVariable.DepartureDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyy-MM-dd', GlobalVariable.DateValue1)

		def Response4
		Response4 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/PNRBFManagement_Add Received info',
				[('Username') : GlobalVariable.Username, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber
					, ('RecordLoc') : GlobalVariable.RecordLoc]))

		println Response4
	}

	@Given ("GWS FareQuoteStandard_32 transaction")
	def GWS_FareQuoteStandard_32_transaction() {

		println ("GWS FareQuoteStandard_32 transaction")
	}




	@Then("FareQuoteStandard_32 Transaction should be successful")
	def FareQuoteStandard_32_Transaction_should_be_successful() {

		//GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue1)

		//GlobalVariable.DepartureDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyy-MM-dd', GlobalVariable.DateValue1)

		def Response5
		Response5 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/FareQuote', [('TokenNumber') : GlobalVariable.TokenNumber]))


		println Response5
	}

	@Given ("GWS PNRBFManagement_53 transaction for PNRBFRetrieveMods")
	def GWS_PNRBFManagement_53_transaction_for_PNRBFRetrieveMods() {

		println ("GWS PNRBFManagement_53 transaction for PNRBFRetrieveMods")
	}

	@When("PNRBFManagement_53 Transaction has RecLoc from PNRBFManagement_49 Response")
	def PNRBFManagement_53_Transaction_has_RecLoc_from_PNRBFManagement_49_Response() {
		//cAPI.BDD_HIS.SetGlobalVariables("abc", "abc")
		println "PNRBFManagement_53 Transaction has RecLoc from PNRBFManagement_49 Response"
	}



	@Then("PNRBFManagement_53 Transaction should be successful")
	def PNRBFManagement_53_Transaction_should_be_successful() {

		//GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue1)

		//GlobalVariable.DepartureDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyy-MM-dd', GlobalVariable.DateValue1)

		def Response6
		Response6 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/PNRRetrieve1', [('Username') : GlobalVariable.Username
			, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber
			, ('RecordLoc') : GlobalVariable.RecordLoc]))

		println Response6
	}

	@Given ("GWS PNRBFManagement_53 transaction for StorePriceMods")
	def GWS_PNRBFManagement_53_transaction_for_StorePriceMods() {

		println ("GWS PNRBFManagement_53 transaction for StorePriceMods")
	}

	@When("PNRBFManagement_53 Transaction has carrier from UAPI LowFareSearch Response")
	def PNRBFManagement_53_Transaction_has_carrier_from_UAPI_LowFareSearch_Response() {
		//cAPI.BDD_HIS.SetGlobalVariables("abc", "abc")
		println "PNRBFManagement_53 Transaction has carrier from UAPI LowFareSearch Response"
	}



	@Then("PNRBFManagement_53 Transaction should be successful for StorePriceMods")
	def PNRBFManagement_53_Transaction_should_be_successful_for_StorePriceMods() {

		//GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue1)

		//GlobalVariable.DepartureDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyy-MM-dd', GlobalVariable.DateValue1)

		def Response7
		Response7 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/PNRBFManagement_StorePrice',
				[('Username') : GlobalVariable.Username, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('Carrier') : GlobalVariable.Carrier
					, ('TokenNumber') : GlobalVariable.TokenNumber]))

		println Response7
	}





	@Then("PNRBFManagement_53 Transaction should be successful PNRBFRetrieveMods after stored fare")
	def PNRBFManagement_53_Transaction_should_be_successful_PNRBFRetrieveMods_after_stored_fare() {

		//GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue1)

		//GlobalVariable.DepartureDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyy-MM-dd', GlobalVariable.DateValue1)

		def Response8
		Response8 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/PNRRetrieve2', [('Username') : GlobalVariable.Username
			, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber
			, ('RecordLoc') : GlobalVariable.RecordLoc]))

		println Response8
	}
	@When("request has session key from BeginSession Response")
	def request_has_session_key_from_BeginSession_Response() {
		//cAPI.BDD_HIS.SetGlobalVariables("abc", "abc")
		println "request has session key from BeginSession Response"
	}



	@Then("EndSession Transaction should be successful")
	def EndSession_Transaction_should_be_successful() {

		//GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue1)

		//GlobalVariable.DepartureDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyy-MM-dd', GlobalVariable.DateValue1)

		def Response9
		Response9 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/Endsession', [('TokenNumber') : GlobalVariable.TokenNumber]))

		println Response9
	}

	@Given ("GWS BeginSession transaction for 1G provider for 8MK PCC")
	def GWS_BeginSession_transaction_for_1G_provider_for_8MK_PCC() {

		println ("GWS BeginSession transaction for 1G provider for 8MK PCC")
	}

	@When("BeginSession with 8MK Profile details")
	def BeginSession_with_8MK_Profile_details() {
		//cAPI.BDD_HIS.SetGlobalVariables("abc", "abc")
		println "BeginSession with 8MK Profile details"
	}



	@Then("BeginSession Transaction should be successful for 8MK PCC")
	def BeginSession_Transaction_should_be_successful_for_8MK_PCC() {

		//GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue1)

		//GlobalVariable.DepartureDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyy-MM-dd', GlobalVariable.DateValue1)

		def Response10
		Response10 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/BeginSession2', [('Profile') : GlobalVariable.Profile2]))

		println Response10
		String xml4 = Response10.responseBodyContent

		def result4 = new XmlSlurper().parseText(xml4)

		GlobalVariable.TokenNumber = result4.BeginSessionResult.text()

		println('TokenNumber' + GlobalVariable.TokenNumber)
	}




	@Then("PNRBFManagement_53 Transaction should be successful2")
	def PNRBFManagement_53_Transaction_should_be_successful2() {

		//GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue1)

		//GlobalVariable.DepartureDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyy-MM-dd', GlobalVariable.DateValue1)

		def Response11
		Response11 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/PNRRetrieve3', [('Username') : GlobalVariable.Username
			, ('PCC') : GlobalVariable.PCC2, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber
			, ('RecordLoc') : GlobalVariable.RecordLoc]))

		println Response11
	}

	@Given ("DocProdFareManipulation_27 transaction for 1G provider")
	def DocProdFareManipulation_27_transaction_for_1G_provider() {

		println ("DocProdFareManipulation_27 transaction for 1G provider")
	}

	@When("request has TransType as TK in TicketingControl")
	def request_has_TransType_as_TK_in_TicketingControl() {
		//cAPI.BDD_HIS.SetGlobalVariables("abc", "abc")
		println "request has TransType as TK in TicketingControl"
	}



	@Then("the DocProdFareManipulation_27 should be successful")
	def the_DocProdFareManipulation_27_should_be_successful() {

		//GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue1)

		//GlobalVariable.DepartureDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyy-MM-dd', GlobalVariable.DateValue1)

		def Response12
		Response12 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/DocProdFareManipulation',
				[('TokenNumber') : GlobalVariable.TokenNumber]))

		println Response12

	}

	@Given ("GWS EndSession transaction")
	def GWS_EndSession_transaction() {

		println ("GWS EndSession transaction")
	}



	@Then("EndSession Transaction should be successful2")
	def EndSession_Transaction_should_be_successful2() {

		//GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue1)

		//GlobalVariable.DepartureDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyy-MM-dd', GlobalVariable.DateValue1)

		def Response13
		Response13 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/Endsession2', [('TokenNumber') : GlobalVariable.TokenNumber]))

		println Response13

	}


}