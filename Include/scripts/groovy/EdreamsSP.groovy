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
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import org.openqa.selenium.Keys as Keys
import groovy.util.XmlSlurper as XmlSlurper
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
import com.travelport.testfactory.*
import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.webui.exception.WebElementNotFoundException
import cucumber.api.java.en.And
import cucumber.api.java.en.Given
import cucumber.api.java.en.Then
import cucumber.api.java.en.When
import smartpoint.*

class EdreamsSP {
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

	//	@And("Launch Smartpoint")
	//	def Launch_Smartpoint() {
	//		println "Launch Smartpoint"
	//
	//		Extbase.setNoProxy()
	//
	//		Windows.startApplicationWithTitle('C:\\fp\\swdir\\viewpoint.exe', '')
	//
	//		//Windows.click(findWindowsObject('Object Repository/terminalSP/Button'))
	//
	//		WebUI.delay(10)
	//
	//		Windows.switchToWindowTitle('Application Window 1')
	//
	//		WebUI.delay(20)
	//
	//
	//
	//	}
	@Given("Launch Smartpoint")
	def launch_Smartpoint() {
		def SPGen = new smartpointGeneric()
		SPGen.LaunchSmartpoint()
	}
	@And("Login Smartpoint")
	def login_Smartpoint() {
		def SPGen = new smartpointGeneric()
		SPGen.LoginSmartpoint()
	}
	@And("Change the currency specific to (.*)")
	//Windows.setText(findWindowsObject('Object Repository/SP_SAT_G/Document'), 'sem/86W0/ag\r\n')
	def change_the_currency_specific_to_LON(String sCtry) {
		def SPGen = new smartpointGeneric()
		SPGen.ChangeCurrency(sCtry)
	}

	@And("Emulate the (.*)")
	def emulate_the_wy(String sPcc) {
		def SPGen = new smartpointGeneric()
		SPGen.EmulatePCC(sPcc)
	}

	@And("Retrieve the ticketed PNR")
	def Retrieve_the_ticketed_PNR() {
		def SPGen = new smartpointGeneric()
		SPGen.RetrivePNR()
	}

	//	@And("Retrieve the ticketed (.*)")
	//	def retrieve_the_ticketed_PNR(String sPNR) {
	//		def SPGen = new smartpointGeneric()
	//		SPGen.RetrivePNR(sPNR)
	//	}

	//	@And("Retrieve the ticketed PNR")
	//	def Retrieve_the_ticketed_PNR() {
	//		println "Retrieve the ticketed PNR"
	//		Windows.setText(findWindowsObject('Object Repository/SP_SAT_G/Document'), '*' + GlobalVariable.PNR)
	//
	//		Windows.sendKeys(findWindowsObject('Object Repository/SP_SAT_G/Document'), Keys.chord(Keys.ENTER))
	//	}


	@When("Change the (.*) of travel for (.*)")
	def change_the_date_of_travel_for_Segment(String sDate, String sSegment) {
		def SATGen = new SATGeneric()
		SATGen.ChangeDateOfTravel(sDate, sSegment)
	}

	@And("Cancel Fares and add new Fares for the PNR")
	def Cancel_Fares_and_add_new_Fares_for_the_PNR() {
		def SATGen = new SATGeneric()
		SATGen.CancelAndAddFares()
	}
	@And("Launch SAT")
	def Launch_SAT() {
		def SATGen = new SATGeneric()
		SATGen.LaunchSAT()
	}

	@And("Navigate to SAT (.*)")
	def navigate_to_Voluntary_Exchange_Tab(String sTab) {
		def SATGen = new SATGeneric()
		SATGen.NavigateToTab(sTab)
	}

	@And("Check the difference in Price")
	def check_the_difference_in_Price() {
		def SATGen = new SATGeneric()
		SATGen.CheckDifferenceInPrice()
		System.out.println('CATEGORY is in  STEP Defenitions: ^^^^^^^^^^^^^^^^^^^^^^'+SATGen.sCategory)
	}

	@And("Perform Voluntary exchange Operations for (.*)")
	def perform_Voluntary_exchange_Operations_for_Customer(String sCustomer) {
		def SATGen = new SATGeneric()
		System.out.println('CATEGORY is in  second STEP Defenitions: ^^^^^^^^^^^^^^^^^^^^^^'+SATGen.sCategory)
		SATGen.VoluntaryExchangeOperation(sCustomer)
	}

	@Then("Validate Ticket status")
	def validate_Ticket_EXCH() {
		def SATGen = new SATGeneric()
		SATGen.ValidateTicketStatus()
	}

	@And("Close SmartPoint")
	def close_SmartPoint() {
		def SPGen = new smartpointGeneric()
		SPGen.CloseSP()
	}

	/* 12Apr'22(SM)-Step definition to fetch ticket details and Validate ticket status from Refund tab prior to refund*/
	@Given("Validate the Retrieved ticket number and Status")
	def validate_the_retrieved_ticket_number_and_Status() {
		def SATGen = new SATGeneric()
		SATGen.ValidateTheRetrievedTicketNumberAndStatus()
	}

	/*  12Apr'22 - (SM)Step definition to perform Refund operation on Refund tab prior to refund */
	@When("Perform (.*) Operation for (.*)")
	def perform_Refund_Operation(String sRefund,String sCustomer){
		def SATGen = new SATGeneric()
		SATGen.PerformRefundOperation(sRefund,sCustomer)
	}

	/*  12Apr'22 - (SM)Step definition to Validate Refunded Ticket status */
	@Then("Validate Refunded Ticket status")
	def validate_Refunded_Ticket_status() {
		def SATGen = new SATGeneric()
		SATGen.ValidateRefundedTicketStatus()
	}

	/*  10May'22 - (Samu)Step definition to Validate Tickets status for ticketed PNR */
	@And("Validate Tickets status for ticketed PNR")
	def Tickete_PNR() {
		def SATGen = new SATGeneric()
		SATGen.TicketedPNRstatus()
	}
	/*  10May'22 - (Samu)Step definition to perform Manual Even exchange */
	@And("perform Manual Even exchange")
	def Manual_Exchanget() {
		def SATGen = new SATGeneric()
		SATGen.ManualEvenExchange()
	}
	/*  10May'22 - (HKN)Step definition to Select Void Check box for Passenger */
	@When("Select Void Check box for Passenger (.*) and Segment (.*)")
	def select_Void_Check_box_for_Passenger_Venkat_and_Segment(String sPaxname,String sSegment) {
		def SATGen = new SATGeneric()
		SATGen.checkVoidCheckBox(sPaxname,sSegment)
	}
	/*  10May'22 - (HKN)Step definition to Perform Void Operations */
	@When("Perform Void Operations")
	def perform_Void_Operations() {
		def SATGen = new SATGeneric()
		SATGen.voidOperation()
	}
	/*  10May'22 - (HKN)Step definition to Validate Ticket details removed from SAT Screen */
	@Then("Validate Ticket details removed from SAT Screen")
	def validate_Ticket_details_removed_from_SAT_Screen() {
		def SATGen = new SATGeneric()
		SATGen.validateTicketDetailsRemovedFromSATScreen()
	}
	/*  10May'22 - (HKN)Step definition to Close SAT */
	@Then("Close SAT")
	def close_SAT() {
		def SATGen = new SATGeneric()
		SATGen.closeSAT()
	}
	/*  10May'22 - (HKN)Step definition to Validate Tickets status for Void */
	@Then("Validate Tickets status for Void")
	def validate_Tickets_status_for_Void() {
		def SATGen = new SATGeneric()
		SATGen.validateTicketsStatusForVoid()
	}
	/*  10May'22 - (HKN)Step definition to Perform Manual Full Refund Operation */
	@Given("Perform Manual Full Refund Operation (.*) for (.*)")
	public void perform_Manual_Full_Refund_Operation_Date_for_Customer(String sDate,String sCustomer) {
		def SATGen = new SATGeneric()
		SATGen.performManualFullRefundOperation(sDate,sCustomer)
	}
	/*  10May'22 - (HKN)Step definition to Validate Tickets status as RFND in *HTE Response */
	@Then("Validate Tickets status as RFND in *HTE Response")
	public void validate_Tickets_status_as_RFND_in_HTE_Response() {
		def SATGen = new SATGeneric()
		SATGen.validateTicketsStatusForRFND()
	}
	/*  31May'22 - (HKN)Step definition to Cancel the Itinerary */
	@And("Cancel the Itinerary")
	public void cancel_the_Itinerary() {
		def SATGen = new SATGeneric()
		SATGen.CancelTheItenary()
	}


}