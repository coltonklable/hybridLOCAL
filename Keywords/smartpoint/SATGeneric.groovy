package smartpoint

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import java.awt.Toolkit as Toolkit
import java.awt.datatransfer.DataFlavor as DataFlavor
//import  java.lang.CharSequence
//import sat.*------
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.testobject.WindowsTestObject as WindowsTestObject
import com.kms.katalon.core.testobject.WindowsTestObject.LocatorStrategy as LocatorStrategy
import java.lang.String as String
import java.awt.datatransfer.StringSelection as StringSelection
import java.awt.datatransfer.*
import com.kms.katalon.core.windows.keyword.helper.WindowsActionHelper as WindowsActionHelper
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import internal.GlobalVariable as GlobalVariable


//import internal.GlobalVariable

public class SATGeneric {
	public static String	sCategory		// To store Voluntary exchange type
	public static String	sCustomer		// To store Customer type
	public String		    sMy_ClipBoard   // To copy Terminal window data
	public static int	    iTotalPaxCount			// To store number of tickets in a PNR
	public static String 	TKTNO			// To Store new ticket value
	//	public static String    str3
	//	public static String    str4
	public static String    aVoidResponse   // To store response received after doing void tickets
	public static Integer   iNoOfTickets    //To store no of tickets present in PNR
	//public static int       iNoOfTickets    //To store no of tickets present in PNR
	public static String[] aTicketNumber = new String[10] // To store all ticket numbers present in a PNR

	@Keyword
	public ChangeDateOfTravel(String sDate, String sSegment) {
		StringBuilder sBuDate  = new StringBuilder()
				.append('@')
				.append(sSegment)
				.append('/')
				.append(sDate)
		System.out.println('Country details :' +sBuDate)
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(sBuDate, Keys.ENTER))
	}

	@Keyword
	public CancelAndAddFares() {
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('R.P', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('ER', Keys.ENTER))
		//		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('ER', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('ER', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('IR', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('FQP1', Keys.ENTER))
		Windows.delay(15)
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('FQP2', Keys.ENTER))
		Windows.delay(15)
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('R.P', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('R.P', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('ER', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('IR', Keys.ENTER))
	}

	@Keyword
	public LaunchSAT() {
		Windows.doubleClick(findWindowsObject('Object Repository/SmartPoint/SAT/Image'))
	}

	public NavigateToTab(String sTab) {
		switch (sTab) {
			case 'VoluntaryChange':
				System.out.println('Selected VoluntaryChangeTab' )
				Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/Tab_Vol'))
				break;
			case 'Void':
				System.out.println('Selected VoidTab' )
				Windows.doubleClick(findWindowsObject('SmartPoint/SAT/Void/Tab_Void'))
				break;
			case 'TicketRefund':
				Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('HMOMC5F749-U', Keys.ENTER))
				System.out.println('Selected Ticket Refund Tab' )
				Windows.doubleClick(findWindowsObject('SmartPoint/SAT/FullRefund/Tab_FullRefund'))
				break
			default:
				System.out.println('Select Valid Tab' )
				break;
		}
	}

	public String CheckDifferenceInPrice() {
		String  	sTabular
		String		sTicketTotal
		String		sAirlineTotal
		Float		fTicketTotal
		Float		fAirlineTotal
		String      sCarrier


		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/LoadTicket'))
		Windows.click(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/LoadTicket'))
		sTabular = Windows.getText(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/FareDifference'))
		if (sTabular == 'Fare Difference') {
			System.out.println('Match found : ' + sTabular)
		}
		sTicketTotal =Windows.getText(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/TicketTotal'))
		sAirlineTotal = Windows.getText(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/AirlineTotal'))
		sCarrier = Windows.getText(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/Carrier'))
		sCarrier=sCarrier.substring(1,3)//returns Carrier Name

		System.out.println('Ticket total as string ------------' + sTicketTotal)
		System.out.println('sAirlineTotal as string ------------'+sAirlineTotal)
		System.out.println('sCarrier as string ------------'+sCarrier)//**

		fTicketTotal =  Float.parseFloat(sTicketTotal.trim())
		fAirlineTotal  =  Float.parseFloat(sAirlineTotal.trim())
		System.out.println('Ticket total as Float ------------' + fTicketTotal)
		System.out.println('sAirlineTotal as float  ------------'+ fAirlineTotal)
		System.out.println('CATEGORY is: %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%'+sCategory)
		if (fTicketTotal == fAirlineTotal) {
			System.out.println('Both Vaules matched : It falls under even exchange')
			System.out.println((('Ticket total is : ' + fTicketTotal) + ' AirlineTotal is : ') + fAirlineTotal)
			sCategory = "EvenExchange"
			System.out.println('CATEGORY is: ^^^^^^^^^^^^^^^^^^^^^^'+sCategory)
			//			EvenExchangeFlow()
		}
		else if (fTicketTotal > fAirlineTotal) {
			System.out.println('Both Vaules not matched : It falls under ADD Collect')
			System.out.println((('Ticket total is : ' + fTicketTotal) + ' AirlineTotal is : ') + fAirlineTotal)
			sCategory = "AddCollect"
			System.out.println('CATEGORY is: ^^^^^^^^^^^^^^^^^^^^^^'+sCategory)
		}
		else if ((fTicketTotal < fAirlineTotal) && ((sCarrier == 'UA')||(sCarrier == 'NZ')) ) {

			//j -y for UA and  Y- B for NZ
			System.out.println('Both Vaules not matched : It falls under ACM')
			System.out.println((('Ticket total is : ' + fTicketTotal) + ' AirlineTotal is : ') + fAirlineTotal)
			sCategory = "ACM"
			System.out.println('CATEGORY is: ^^^^^^^^^^^^^^^^^^^^^^'+sCategory)
		}
		else if (fTicketTotal < fAirlineTotal) {
			System.out.println('Both Vaules not matched : It falls under EMDRB')
			System.out.println((('Ticket total is : ' + fTicketTotal) + ' AirlineTotal is : ') + fAirlineTotal)
			sCategory = "EMDRB"
			System.out.println('CATEGORY is: ^^^^^^^^^^^^^^^^^^^^^^'+sCategory)
		}
		return sCategory
	}

	public VoluntaryExchangeOperation(String sCustomer) {
		System.out.println('CATEGORY in Switch (voluntary exchange operation) class: ********************'+sCategory)
		//		sCategory = 'AddCollect'
		switch (sCategory) {
			case 'EvenExchange':
				System.out.println('Even Exchange flow is in progress' )
				EvenExchangeFlow()
				break;
			case 'AddCollect':
				System.out.println('ADD Collect flow is in progress' )
				AddCollectFlow()
				break;
			case 'EMDRB':
				System.out.println('EMDRB flow is in progress' )
				EMDRBFlow()
				break;
			case 'ACM':
				System.out.println('ACM flow is in progress' )
				ACMFlow(sCustomer)
				break;
			default:
				System.out.println('Not falling under Even Exchange, ADCOL, EMDRB or ACM' )
				break;
		}
		return sCustomer
	}

	public EvenExchangeFlow() {
		String sStatus

		//	verify Validate rules check box and select the same
		Windows.verifyElementPresent(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'), 5)
		Windows.click(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'))
		sStatus = Windows.getAttribute(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'), 'Toggle.ToggleState')
		System.out.println('status of the validate check box on first instance is  : '+sStatus)
		System.out.println('status of the validate check box on first instance is  : '+sStatus)
		if (Integer.parseInt(sStatus) == 1 )
		{
			System.out.println('Validate Rules check box is selected on single click : '+sStatus)
		}
		else
		{
			Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'))
			sStatus = Windows.getAttribute(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'), 'Toggle.ToggleState')
			System.out.println('status of the validate check box second instance is  : '+sStatus)
			if (Integer.parseInt(sStatus) == 1)
			{
				System.out.println('Validate Rules check box is selected on Double click : '+sStatus)
			}
		}
		//	Click Exchange ticket button
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ExchangeTicket'))
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/OkManualFare'))
		WebUI.delay(5)
		//	Click Continue button in EX ticket screen
		Windows.verifyElementPresent(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/WinAdditionalPayment'), 5)
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ContinueEXTicket'))
		WebUI.delay(2)
		//	Click Continue button in TP ticket screen
		Windows.verifyElementPresent(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/WinAdditionalPayment'), 5)
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ContinueTPTicket'))
		//	Click Update Exchange and close button
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/UpdateExchangeClose'))
		WebUI.delay(1)
	}
	public AddCollectFlow() {
		/* ********************************************************************************************************************
		 FUNCTION    : AddCollectFlow()
		 AUTHOR      : Samundeswari K
		 DESCRIPTION : Function to Add Collect operation
		 DATE        : 12'Apr22
		 EXAMPLE     : AddCollectFlow()
		 *********************************************************************************************************************/
		String sStatus  //Object to Identify SAT Status

		//	verify Validate rules check box and select the same
		Windows.verifyElementPresent(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'), 5)
		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'), Keys.chord( Keys.SPACE))
		sStatus = Windows.getAttribute(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'), 'Toggle.ToggleState')
		System.out.println('status of the validate check box on first instance is  : '+sStatus)
		if (Integer.parseInt(sStatus) == 1 )
		{
			System.out.println('Validate Rules check box is selected on single click : '+sStatus)
		}
		else
		{
			Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'))
			sStatus = Windows.getAttribute(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'), 'Toggle.ToggleState')
			System.out.println('status of the validate check box second instance is  : '+sStatus)
			if (Integer.parseInt(sStatus) == 1)
			{
				System.out.println('Validate Rules check box is selected on Double click : '+sStatus)
			}
		}
		//	Click Exchange ticket button
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ExchangeTicket'))
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/OkManualFare'))
		WebUI.delay(3)
		//	Click Continue button in EX ticket screen
		Windows.verifyElementPresent(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/WinAdditionalPayment'), 5)
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ContinueEXTicket'))
		WebUI.delay(1)
		//	Click Continue button in TP ticket screen
		Windows.verifyElementPresent(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/WinAdditionalPayment'), 5)
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ContinueTPTicket'))
		//	Click Continue button in Add collect ticket screen
		Windows.verifyElementPresent(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/WinAdditionalPayment'), 5)
		Windows.doubleClick(findWindowsObject('Object Repository/SmartPoint/SAT/AddCollect/ButtonContinueAddAddCollect'))
		//	Click Update Exchange and close button
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/UpdateExchangeClose'))
		WebUI.delay(1)
	}

	public EMDRBFlow() {
		/* *******************************************************************************************************************
		 Function    : EMDRBFlow()
		 AUTHOR      :  Nishanth.Hk
		 Description : Function to perform EMDRB Exchange
		 Date        : 10'May22
		 Example     : EMDRBFlow()
		 *********************************************************************************************************************/
		String sStatus
		String sEmdlButton

		//	verify Validate rules check box and select the same
		Windows.verifyElementPresent(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'), 5)
		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'), Keys.chord( Keys.SPACE))
		sStatus = Windows.getAttribute(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'), 'Toggle.ToggleState')
		System.out.println('status of the validate check box on first instance is  : '+sStatus)
		System.out.println('status of the validate check box on first instance is  : '+sStatus)
		if (Integer.parseInt(sStatus) == 1 )
		{
			System.out.println('Validate Rules check box is selected on sending Key SPACE : '+sStatus)
		}
		//	Click Exchange ticket button
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ExchangeTicket'))
		// Click OK Button
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/OkManualFare'))
		WebUI.delay(5)
		//	verifyElementPresent in WinAdditionalPayment screen
		assert Windows.verifyElementPresent(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/WinAdditionalPayment'), 5)
		//Click  first Continue button
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ContinueEXTicket'))
		WebUI.delay(2)
		//	verifyElementPresent in WinAdditionalPayment screen
		assert Windows.verifyElementPresent(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/WinAdditionalPayment'), 5)
		// Click  Second Continue button
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ContinueTPTicket'))
		WebUI.delay(3)
		//	verifyElementPresent in WinAdditionalPayment screen
		assert Windows.verifyElementPresent(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/WinAdditionalPayment'), 5)
		//  Enter 'Y' in textBox
		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/TextBoxCreateEMD'), Keys.chord('Y'))
		// Click Continue button

		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ContinueEMD'))
		WebUI.delay(2)
		// Enter 'TEST' in TextFieldEndorsement
		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/TextFieldEndorsement'), Keys.chord('TEST'))
		//	Click ContinueEndorsement
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ContinueEndorsement'))
		WebUI.delay(2)
		// Click UpdateExchangeClose button
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/UpdateExchangeClose'))
		WebUI.delay(2)
		//Verify EMDL Button present in PnrViewr or Not
		sEmdlButton = Windows.getAttribute(findWindowsObject('SmartPoint/SAT/ButtonsPnrViewer/ButtonEMDL'),'IsOffscreen')
		System.out.println(sEmdlButton)//False
		if(sEmdlButton == 'False')
		{
			System.out.println('Button EMDL Present in PNR Viewer')

		}
		else
		{
			System.out.println('Button EMDL not Present in PNR Viewer')
			assert false
		}
	}


	public ValidateTicketStatus()
	{
		String sTktStatusOld
		String sTktStatusNew
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('*HTE', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('*TE001', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord(Keys.ALT, 'c'))
		String my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		System.out.println(my_clipboard)
		System.out.println('Contains: ' + my_clipboard.contains('EXCH'))
		sTktStatusOld = my_clipboard.contains('EXCH')
		if (sTktStatusOld == true) {
			System.out.println('Match found for Exchange : ' + sTktStatusOld)
		}
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('*TE002', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord(Keys.ALT, 'c'))
		my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		System.out.println(my_clipboard)
		System.out.println('Contains: ' + my_clipboard.contains('OPEN'))
		sTktStatusNew = my_clipboard.contains('OPEN')
		if (sTktStatusNew == true) {
			System.out.println('Match found for Open : ' + sTktStatusNew)
		}

		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('XI', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('R.P+ER', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('ER', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('IR', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('I', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('I', Keys.ENTER))
	}
	public CopyClipboard()
	{
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.ALT, 'c'))
		sMy_ClipBoard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		System.out.println(sMy_ClipBoard)
		System.out.println('Contains: ' + sMy_ClipBoard.contains('EXCH'))
		return(sMy_ClipBoard)
	}


	public ACMFlow(sCustomer) {
		/* *******************************************************************************************************************
		 Function    : ACMFlow()
		 AUTHOR      :  Nishanth.Hk
		 Description : Function to ACM Exchange
		 Date        : 10'May22
		 Example     : ACMFlow()
		 *********************************************************************************************************************/
		String sStatus

		//	verify Validate rules check box and select the same
		Windows.verifyElementPresent(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'), 5)
		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'), Keys.chord( Keys.SPACE))
		//Windows.verifyElementPresent(findWindowsObject('SP_SAT_Inspect/ValidateRulesCopy'), 5)
		sStatus = Windows.getAttribute(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'), 'Toggle.ToggleState')
		System.out.println('status of the validate check box on first instance is  : '+sStatus)
		System.out.println('status of the validate check box on first instance is  : '+sStatus)
		if (Integer.parseInt(sStatus) == 1 )
		{
			System.out.println('Validate Rules check box is selected on sending Key SPACE : '+sStatus)
		}
		else
		{
			Windows.click(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'))
			sStatus = Windows.getAttribute(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ValidateRulesCopy'), 'Toggle.ToggleState')
			System.out.println('status of the validate check box second instance is  : '+sStatus)
			if (Integer.parseInt(sStatus) == 1)
			{
				System.out.println('Validate Rules check box is selected on Single click : '+sStatus)
			}
		}
		WebUI.delay(2)

		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ExchangeTicket'))

		WebUI.delay(3)

		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/OkManualFare'))

		WebUI.delay(5)
		if(sCustomer == 'Delta Vacations') {

			Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/OkManualFare'))
			WebUI.delay(5)
		}


		Windows.verifyElementPresent(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/WinAdditionalPayment'), 5)

		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ContinueEXTicket'))

		WebUI.delay(2)

		Windows.verifyElementPresent(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/WinAdditionalPayment'), 5)

		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ContinueTPTicket'))
		WebUI.delay(2)
		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/TextBoxACM'), Keys.chord('Y'))
		Windows.verifyElementPresent(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/WinAdditionalPayment'), 5)
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/ContinueACM'))
		WebUI.delay(2)
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/VoluntaryExchangeTab/UpdateExchangeClose'))
		WebUI.delay(2)

	}


	public ValidateTheRetrievedTicketNumberAndStatus()
	{
		/* *******************************************************************************************************************
		 FUNCTION    : ValidateTheRetrievedTicketNumberAndStatus()
		 AUTHOR      : SathyaMothilal
		 DESCRIPTION : Function to fetch ticket details and Validate ticket status from Refund tab prior to refund
		 DATE        : 12'Apr22
		 EXAMPLE     : ValidateTheRetrievedTicketNumberAndStatus()
		 *********************************************************************************************************************/


		String sPassengerName 			// Passenger Name
		String sTicketNumber			// Ticket Number
		String sCoupon1Status			// Coupon 1 Status
		String sCoupon2Status			// Coupon 2 Status
		String[] aPassengerNameList		// Array of Passenger Name List
		String[] aTicketNumberList		// Array of Ticket Number List
		String[] aCoupon1StatusList		// Array of Coupon 1 Status List
		String[] aCoupon2StatusList		// Array of Coupon 2 Status List



		//Fetch the Refund tab table details
		// Assigning the passenger names identified in a List
		WindowsTestObject po1 = new WindowsTestObject('objectName1')
		sPassengerName = '//Text[@AutomationId="TextBlock_2ec93186271c1b8"]'
		po1.setLocatorStrategy(LocatorStrategy.XPATH)
		po1.setLocator(sPassengerName)
		List<WebElement> PassengerNameList = Windows.findElements(po1)
		iTotalPaxCount = PassengerNameList.size()
		System.out.println('************  Number of tickets issued for the current PNR  : ' + iTotalPaxCount )

		// Assigning the ticket numbers identified in a List
		WindowsTestObject to1 = new WindowsTestObject('objectName1')
		sTicketNumber = '//Text[@AutomationId="TextBlock_2ec93186271c342"]'
		to1.setLocatorStrategy(LocatorStrategy.XPATH)
		to1.setLocator(sTicketNumber)
		List<WebElement> TicketNumberList = Windows.findElements(to1)

		//Assigning the Coupon 1 Status identified in a List
		WindowsTestObject co1 = new WindowsTestObject('objectName1')
		sCoupon1Status = '//Text[@AutomationId="TextBlock_2ec93186271c787"]'
		co1.setLocatorStrategy(LocatorStrategy.XPATH)
		co1.setLocator(sCoupon1Status)
		List<WebElement> Coupon1StatusList = Windows.findElements(co1)

		//Assigning the Coupon 1 Status identified in a List
		WindowsTestObject co2 = new WindowsTestObject('objectName1')
		sCoupon2Status = '//Text[@AutomationId="TextBlock_2ec93186271c787"]'
		co2.setLocatorStrategy(LocatorStrategy.XPATH)
		co2.setLocator(sCoupon2Status)
		List<WebElement> Coupon2StatusList = Windows.findElements(co2)


		aPassengerNameList = new String[iTotalPaxCount]
		aTicketNumberList = new String[iTotalPaxCount]
		aCoupon1StatusList = new String[iTotalPaxCount]
		aCoupon2StatusList = new String[iTotalPaxCount]

		//Fetch and Print the values of Ticket Details
		for (int i = 0; i < iTotalPaxCount; i++)
		{
			(aPassengerNameList[i]) = (PassengerNameList[i]).getText()
			(aTicketNumberList[i]) = (TicketNumberList[i]).getText()
			(aCoupon1StatusList[i]) = (Coupon1StatusList[i]).getText()
			(aCoupon2StatusList[i]) = (Coupon2StatusList[i]).getText()
			if (((aCoupon1StatusList[i]) == "OPEN") && ((aCoupon2StatusList[i]) == "OPEN"))
			{
				System.out.println(((' *********   Name of Passenger ' + (i + 1)) + ' is ') + (aPassengerNameList[i]))
				System.out.println(((' *********   Ticket Number of Passenger ' + (i + 1)) + ' is ') + (aTicketNumberList[i]))
				System.out.println(((' *********   Coupon 1 Status of Passenger ' + (i + 1)) + ' is ') + (aCoupon1StatusList[i]))
				System.out.println(((' *********   Coupon 2 Status of Passenger ' + (i + 1)) + ' is ') + (aCoupon2StatusList[i]))
			}
			else
			{
				System.out.println("*************  Verify the Coupon1 Status in the Ticket Refund tab "+ (aCoupon1StatusList[i]))
				System.out.println("*************  Verify the Coupon2 Status in the Ticket Refund tab "+ (aCoupon2StatusList[i]))
			}

		}
		return iTotalPaxCount
		Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/ButtonCancel'))

	}


	public PerformRefundOperation(sRefund,sCustomer)
	{
		/* ********************************************************************************************************************
		 FUNCTION    : PerformRefundOperations(sRefund,sCustomer)
		 AUTHOR      : Sathya Mothilal
		 DESCRIPTION : Function to perform Refund operation
		 DATE        : 12'Apr22 
		 EXAMPLE     : PerformRefundOperations(FULLREFUND,EDreams)
		 sRefund is Refund Type
		 sCustomer is the Customer Type
		 *********************************************************************************************************************/

		switch (sRefund)
		{
			case 'FULLREFUND':
				System.out.println(' Selected Full Refund Flow' )
				FullRefundFlow(sCustomer)
				break;
			case 'PARTIALREFUND':
				System.out.println(' Selected Partial Refund Flow' )
				PartialRefundFlow()
				break;
			default:
				System.out.println('Select Valid Tab' )
				break;
		}
	}

	public FullRefundFlow(String sCustomer)
	{
		/* ********************************************************************************************************************
		 FUNCTION    : FullRefundFlow()
		 AUTHOR      : Sathya Mothilal
		 DESCRIPTION : Function to perform Full Refund operation
		 DATE        : 12'Apr22
		 EXAMPLE     : FullRefundFlow()
		 **********************************************************************************************************************/

		String sFullRefundResponse  	// Successful Full Refund Response
		int iRefundsize					// Successful Full Refund Response Count
		String[] asFulllRefundResponse	// Array to store Full Refund Response
		String sCurrencyCode			// PCC Currency Code
		String sFareAmount				// Total Fare Amount
		int iRefundEntries = 0

		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('*HTE', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord(Keys.ALT, 'c'))
		String my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)

		if (my_clipboard.contains('OPEN'))
		{
			System.out.println('*************** This is a single ticket PNR ***************')
			System.out.println(my_clipboard)
			int iIndex1 = my_clipboard.indexOf('TOTAL')
			System.out.println(iIndex1)
			sCurrencyCode = my_clipboard.substring(iIndex1 + 6, iIndex1 + 9)
			System.out.println(sCurrencyCode)
			sFareAmount = ((my_clipboard.substring((iIndex1+9),(iIndex1+18)))).trim()
			System.out.println("*************** Total Fare Amount ***************"+ sFareAmount + sCurrencyCode)
			WebUI.delay(3)
			System.out.println("Click SAT Icon")
			Windows.click(findWindowsObject('SmartPoint/SAT/Image'))
			System.out.println("Click SAT Ticket Refund Tab")
			Windows.doubleClick(findWindowsObject('SmartPoint/SAT/FullRefund/Tab_FullRefund'))
			System.out.println("Select Full Refund Checkbox")
			Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/CheckBoxFullRefund'))
			System.out.println("Click Confirm Button")
			Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/Continue1FullRefundTicket'))
			System.out.println("Click Yes Button")
			Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/ButtonYes'))
			WebUI.delay(20)

			WindowsTestObject ro1 = new WindowsTestObject('objectName1')
			if(sCustomer == 'EDreams')
			{
				sFullRefundResponse = '//Text[@AutomationId="TextBlock_2ec9318626e979e"]'
			}
			else if(sCustomer == 'DeltaVacations')
			{
				sFullRefundResponse = '//Text[@AutomationId="TextBlock_2ec9318626e9d57"]'
			}

			ro1.setLocatorStrategy(LocatorStrategy.XPATH)
			ro1.setLocator(sFullRefundResponse)
			List<WebElement> foundElements4 = Windows.findElements(ro1)
			String sFulllRefundResponse = (foundElements4[1]).getText()
			System.out.println(sFulllRefundResponse)

			if ((sFulllRefundResponse.contains('AUTOMATED REFUND NOTICE ISSUEDCASH REFUND AMOUNT')) || (sFulllRefundResponse.contains('REFUND COMPLETECASH REFUND AMOUNT')))
			{
				if (sFulllRefundResponse.contains(sFareAmount))
				{
					System.out.println('**** Successful Refund Response Displayed in the Refund Screen ' + (sFulllRefundResponse))
				}
			}
			else
			{
				System.out.println(' ******** Refund not successful, please verify the message on the refund window   ')
			}

			Windows.click(findWindowsObject('SmartPoint/SAT/PartialRefund/OKButton_PartRefund'))
			WebUI.delay(3)
			Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/ButtonCancel'))
			Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('IR', Keys.ENTER))

		}

		else if (my_clipboard.contains('*TE00'))
		{
			for (int i=1;i<=iTotalPaxCount;i++)
			{
				Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('*TE00'+i, Keys.ENTER))
				Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord(Keys.ALT, 'c'))
				String my_clipboard1 = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
				System.out.println(my_clipboard)
				if (my_clipboard1.contains('OPEN'))
				{
					int iIndex1 = my_clipboard1.indexOf('TOTAL')
					System.out.println(iIndex1)
					sCurrencyCode = my_clipboard1.substring(iIndex1 + 6, iIndex1 + 9)
					System.out.println(sCurrencyCode)
					sFareAmount = ((my_clipboard1.substring((iIndex1+9),(iIndex1+18)))).trim()
					System.out.println(" Total Fare Amount  "+ sFareAmount + sCurrencyCode)
					WebUI.delay(3)
					Windows.click(findWindowsObject('SmartPoint/SAT/Image'))
					WebUI.delay(3)
					Windows.doubleClick(findWindowsObject('SmartPoint/SAT/FullRefund/Tab_FullRefund'))

					//Identifying the Full refund Check box
					WindowsTestObject po2 = new WindowsTestObject('objectName1')
					String sFullRefundCheckbox = '//CheckBox[@AutomationId="CheckBox_2ec93186271d33e"]'
					po2.setLocatorStrategy(LocatorStrategy.XPATH)
					po2.setLocator(sFullRefundCheckbox)
					List<WebElement> foundElements10 = Windows.findElements(po2)
					System.out.println("Select the Full Refund Checkbox ")
					foundElements10[(i-1)].sendKeys(Keys.chord(Keys.SPACE))


					//Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/CheckBoxFullRefund'))
					System.out.println("Click Continue Button")
					Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/Continue1FullRefundTicket'))
					System.out.println("Click Yes Button")
					Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/ButtonYes'))
					WebUI.delay(20)

					WindowsTestObject ro1 = new WindowsTestObject('objectName1')
					if (sCustomer == 'EDreams')
					{
						sFullRefundResponse = '//Text[@AutomationId="TextBlock_2ec9318626e979e"]'
					}
					else if (sCustomer == 'DeltaVacations')
					{
						sFullRefundResponse = '//Text[@AutomationId="TextBlock_2ec9318626e9d57"]'
					}

					ro1.setLocatorStrategy(LocatorStrategy.XPATH)
					ro1.setLocator(sFullRefundResponse)
					List<WebElement> foundElements4 = Windows.findElements(ro1)
					String sFulllRefundResponse = (foundElements4[1]).getText()
					System.out.println(sFulllRefundResponse)

					if ((sFulllRefundResponse.contains('AUTOMATED REFUND NOTICE ISSUEDCASH REFUND AMOUNT')) || (sFulllRefundResponse.contains('REFUND COMPLETECASH REFUND AMOUNT')))
					{
						if (sFulllRefundResponse.contains(sFareAmount))
						{
							System.out.println('Successful Refund Response Displayed in the Refund Screen ' + (sFulllRefundResponse))
						}
					}
					else
					{
						System.out.println('Refund not successful, please verify the message on the refund window   ')
					}

					Windows.click(findWindowsObject('SmartPoint/SAT/PartialRefund/OKButton_PartRefund'))
					WebUI.delay(3)
					Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/ButtonCancel'))
					Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('IR', Keys.ENTER))

				}
			}
		}

	}



	public ValidateRefundedTicketStatus()
	{
		/* *******************************************************************************************************************
		 FUNCTION    : ValidateRefundedTicketStatus()
		 AUTHOR      : Sathya Mothilal
		 DESCRIPTION : Function to fetch ticket details and Validate ticket status After Refund
		 DATE        : 12'Apr22 
		 EXAMPLE     : ValidateRefundedTicketStatus()
		 *********************************************************************************************************************/		

		int iRefundCount					// Number of tickets refunded in a PNR
		String sCoupon1						// Coupon 1 Status
		String sCoupon2						// Coupon 2 Status
		String[] aCoupon1RefundStatusList   // Array to Store Coupon 1 Status
		String[] aCoupon2RefundStatusList   // Array to Store Coupon 2 Status

		WebUI.delay(3)
		Windows.click(findWindowsObject('SmartPoint/SAT/Image'))
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/FullRefund/Tab_FullRefund'))

		// Assigning Coupon 1 Status of Refunded Ticket to a list
		WindowsTestObject co3 = new WindowsTestObject('objectName1')
		sCoupon1 = '//Text[@AutomationId="TextBlock_2ec93186271c787"]'
		co3.setLocatorStrategy(LocatorStrategy.XPATH)
		co3.setLocator(sCoupon1)
		List<WebElement> Coupon1List = Windows.findElements(co3)
		iRefundCount = Coupon1List.size()

		//Assigning Coupon 2 Status of Refunded Ticket to a list
		WindowsTestObject co4 = new WindowsTestObject('objectName1')
		sCoupon2 = '//Text[@AutomationId="TextBlock_2ec93186271c787"]'
		co4.setLocatorStrategy(LocatorStrategy.XPATH)
		co4.setLocator(sCoupon2)
		List<WebElement> Coupon2List = Windows.findElements(co4)

		aCoupon1RefundStatusList = new String[iRefundCount]
		aCoupon2RefundStatusList = new String[iRefundCount]

		// Print the values of Refunded Ticket Details
		for (int i = 0; i < iRefundCount; i++)
		{
			(aCoupon1RefundStatusList[i]) = (Coupon1List[i]).getText()
			(aCoupon2RefundStatusList[i]) = (Coupon2List[i]).getText()
			if (((aCoupon1RefundStatusList[i]) == "RFND") && ((aCoupon2RefundStatusList[i]) == "RFND"))
			{
				System.out.println(((' *********   Coupon 1 Status of Passenger ' + (i + 1)) + ' after Refund is ') + (aCoupon1RefundStatusList[i]))
				System.out.println(((' *********   Coupon 2 Status of Passenger ' + (i + 1)) + ' after Refund is ') + (aCoupon2RefundStatusList[i]))

			}
			else
			{
				System.out.println("*******  Verify the Coupon1 Status in the Ticket Refund tab "+ (aCoupon1RefundStatusList[i]))
				System.out.println("*******  Verify the Coupon2 Status in the Ticket Refund tab "+ (aCoupon2RefundStatusList[i]))
			}
		}
		Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/ButtonCancel'))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('XI', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('R.P+ER', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('ER', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('IR', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('I', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('I', Keys.ENTER))
	}

	public PartialRefundFlow()
	{

		/* *******************************************************************************************************************
		 FUNCTION    : PartialRefundFlow()
		 AUTHOR      : Sathya Mothilal
		 DESCRIPTION : Function to perform Partial Refund operation
		 DATE        : 22'Apr22
		 EXAMPLE     : PartialRefundFlow()
		 *********************************************************************************************************************/
		String sTextbox					// Object to identify Text box
		String sRefundResponse			// Object to identify refund response
		String sCheckbox				// Object to identify Check box
		String sCurrencyCode			// Currency Code of PCC
		String sFareAmount				// Total Fare Amount
		String sRefundAmount			// Conversion of Refund Value
		String sFinalRefundAmount		// Final Refund Amount from *HTE response to be validated with SAT window response
		String sFinalRefundAmountSAT	// Final Refund Amount from SAT Window
		int iDecimalvalue
		int iIndex1, iIndex2
		int iRefundAmountlength
		float fRefundCalculation1, fRefundCalculation

		Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/ButtonCancel'))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('*HTE', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord(Keys.ALT, 'c'))
		String my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		if (my_clipboard.contains('OPEN'))
		{
			System.out.println('*************** This is a single ticket PNR ***************')
			System.out.println(my_clipboard)
			iIndex1 = my_clipboard.indexOf('TOTAL')
			System.out.println(iIndex1)
			sCurrencyCode = my_clipboard.substring(iIndex1 + 6, iIndex1 + 9)
			System.out.println(sCurrencyCode)
			sFareAmount = (my_clipboard.substring(iIndex1 + 9,iIndex1+18)).trim()
			//			System.out.println(sFareAmount)
			System.out.println(" Total Fare Amount "+" "+ sFareAmount +" "+ sCurrencyCode)

			fRefundCalculation1=(Float.parseFloat(sFareAmount))-(50.00)-(10.00)
			System.out.println(fRefundCalculation1)
			fRefundCalculation=Math.round(fRefundCalculation1*100.0)/100.0
			sRefundAmount = String.valueOf(fRefundCalculation)
			System.out.println(sRefundAmount)
			iRefundAmountlength = sRefundAmount.length()
			iIndex2 = sRefundAmount.indexOf('.')

			iDecimalvalue = (iRefundAmountlength)-(iIndex2)
			sFinalRefundAmount = sRefundAmount
			if(iDecimalvalue == 2)
			{
				sFinalRefundAmount = sRefundAmount + '0'
				System.out.println("  The final Refund Amount to be verified in SAT Window Response   " + sFinalRefundAmount )
			}
			else
			{
				System.out.println("  The final Refund Amount to be verified in SAT Window Response " + sFinalRefundAmount )
			}


			System.out.println("Click SAT Icon")
			Windows.doubleClick(findWindowsObject('SmartPoint/SAT/Image'))
			System.out.println("Click SAT_Ticket Refund Tab ")
			Windows.doubleClick(findWindowsObject('SmartPoint/SAT/FullRefund/Tab_FullRefund'))
			System.out.println("Select the Partial Refund Checkbox")
			Windows.sendKeys(findWindowsObject('SmartPoint/SAT/PartialRefund/CheckBoxPartialRefund'), Keys.chord( Keys.SPACE))
			System.out.println("Click on CONTINUE Button")
			Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/Continue1FullRefundTicket'))
			WebUI.delay(3)
			System.out.println("Click on YES Button")
			Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/ButtonYes'))
			WebUI.delay(3)

			//Enter the cash amount used and refund amount used in 1st screen of refund window
			WindowsTestObject ro1 = new WindowsTestObject("objectName1")
			sTextbox = '//Edit[@AutomationId="TextBox_2ec9318626cd4b2"]'
			ro1.setLocatorStrategy(LocatorStrategy.XPATH)
			ro1.setLocator(sTextbox)
			List<WebElement> TextboxList = Windows.findElements(ro1)
			System.out.println("Enter the Cash Amount Used 50.00  && Cancellation Charge 10.00")
			TextboxList[1].sendKeys(Keys.chord('50.00'))
			TextboxList[3].sendKeys(Keys.chord('10.00'))
			WebUI.delay(5)
			System.out.println("Click on CONFIRM Button")
			Windows.click(findWindowsObject('SmartPoint/SAT/PartialRefund/ButtonConfirmPartialRefund'))
			WebUI.delay(15)
			System.out.println("Click on CONFIRM Button")
			Windows.click(findWindowsObject('SmartPoint/SAT/PartialRefund/ButtonConfirmPartialRefund'))
			WebUI.delay(3)

			//Fetch the Refund amount displayed in the refund window
			sRefundResponse = '//Text[@AutomationId="TextBlock_2ec9318626cb67e"]'
			WindowsTestObject ro2 = new WindowsTestObject("objectName3")
			ro2.setLocatorStrategy(LocatorStrategy.XPATH)
			ro2.setLocator(sRefundResponse)
			List<WebElement> sRefundResponseList = Windows.findElements(ro2)
			String sRefundResponse1 = sRefundResponseList[1].getText()
			System.out.println(sRefundResponse1)
			sFinalRefundAmountSAT = (sRefundResponse1.substring(24,35)).trim()
			System.out.println("Refund Amount displayed in the final screen of SAT Window after Partial Refund   " + sFinalRefundAmountSAT )//1109.00

			//Validate if the refund amount calculated from the Terminal window matches with the refund value from the SAT partial refund flow
			if(sFinalRefundAmount == sFinalRefundAmountSAT)
			{
				System.out.println(" Total fare before Refund is  "+sFareAmount+" and after Partial refund Refundable amount is  "+sFinalRefundAmountSAT)
				System.out.println("*********** Click on CONFIRM Button***************")
				Windows.click(findWindowsObject('SmartPoint/SAT/PartialRefund/ButtonConfirmPartialRefund'))
				WebUI.delay(5)
				Windows.click(findWindowsObject('SmartPoint/SAT/PartialRefund/ButtonFinalOK'))
				Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/ButtonCancel'))

				//Validate the Ticket Status in *HTE response
				Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('IR', Keys.ENTER))
			}
			else
			{
				System.out.println("Refundable amount is not correct")
				assert false
			}
		}
		else if (my_clipboard.contains('*TE00'))
		{
			for (int i=1;i<=iTotalPaxCount;i++)
			{
				Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('*TE00'+i, Keys.ENTER))
				Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord(Keys.ALT, 'c'))
				String my_clipboard1 = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
				if (my_clipboard1.contains('OPEN'))
				{
					System.out.println(my_clipboard1)
					iIndex1 = my_clipboard1.indexOf('TOTAL')
					System.out.println(iIndex1)
					sCurrencyCode = my_clipboard1.substring(iIndex1 + 6, iIndex1 + 9)
					System.out.println(sCurrencyCode)
					sFareAmount = (my_clipboard1.substring(iIndex1 + 9,iIndex1+18)).trim()
					System.out.println(sFareAmount)
					System.out.println("Total Fare Amount  "+ sFareAmount +"  " +sCurrencyCode)

					fRefundCalculation1=(Float.parseFloat(sFareAmount))-(50.00)-(10.00)
					System.out.println(fRefundCalculation1)
					fRefundCalculation=Math.round(fRefundCalculation1*100.0)/100.0
					sRefundAmount = String.valueOf(fRefundCalculation)
					System.out.println(sRefundAmount)
					iRefundAmountlength = sRefundAmount.length()
					iIndex2 = sRefundAmount.indexOf('.')

					iDecimalvalue = (iRefundAmountlength)-(iIndex2)
					sFinalRefundAmount = sRefundAmount
					if(iDecimalvalue == 2)
					{
						sFinalRefundAmount = sRefundAmount + '0'
						System.out.println(" The final Refund Amount to be verified in SAT Window Response " + sFinalRefundAmount )
					}
					else
					{
						System.out.println(" The final Refund Amount to be verified in SAT Window Response  " + sFinalRefundAmount )
					}

					System.out.println("Click SAT Icon")
					Windows.doubleClick(findWindowsObject('SmartPoint/SAT/Image'))
					System.out.println("Click SAT_Ticket Refund Tab")
					Windows.doubleClick(findWindowsObject('SmartPoint/SAT/FullRefund/Tab_FullRefund'))

					//Identifying the partial refund Check box
					WindowsTestObject po2 = new WindowsTestObject('objectName1')
					sCheckbox = '//CheckBox[@AutomationId="CheckBox_2ec93186271d57f"]'
					po2.setLocatorStrategy(LocatorStrategy.XPATH)
					po2.setLocator(sCheckbox)
					List<WebElement> CheckboxList = Windows.findElements(po2)
					System.out.println("Select the Partial Refund Checkbox")
					CheckboxList[(i-1)].sendKeys(Keys.chord(Keys.SPACE))
					System.out.println("Click on CONTINUE Button")
					Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/Continue1FullRefundTicket'))
					WebUI.delay(3)
					System.out.println("Click the YES Button")
					Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/ButtonYes'))
					WebUI.delay(3)

					//Enter the cash amount used and refund amount used in 1st screen of refund window
					WindowsTestObject ro1 = new WindowsTestObject("objectName1")
					sTextbox = '//Edit[@AutomationId="TextBox_2ec9318626cd4b2"]'
					ro1.setLocatorStrategy(LocatorStrategy.XPATH)
					ro1.setLocator(sTextbox)
					List<WebElement> TextboxList = Windows.findElements(ro1)
					System.out.println("Enter the Cash Amount Used 50.00  && Cancellation Charge 10.00")
					TextboxList[1].sendKeys(Keys.chord('50.00'))
					TextboxList[3].sendKeys(Keys.chord('10.00'))
					WebUI.delay(5)
					System.out.println("Click on CONFIRM Button")
					Windows.click(findWindowsObject('SmartPoint/SAT/PartialRefund/ButtonConfirmPartialRefund'))
					WebUI.delay(15)
					System.out.println("Click on CONFIRM Button")
					Windows.click(findWindowsObject('SmartPoint/SAT/PartialRefund/ButtonConfirmPartialRefund'))
					WebUI.delay(3)

					//Fetch the Refund amount displayed in the refund window
					sRefundResponse = '//Text[@AutomationId="TextBlock_2ec9318626cb67e"]'
					WindowsTestObject ro2 = new WindowsTestObject("objectName3")
					ro2.setLocatorStrategy(LocatorStrategy.XPATH)
					ro2.setLocator(sRefundResponse)
					List<WebElement> RefundResponseList = Windows.findElements(ro2)
					String sRefundResponse1 = RefundResponseList[1].getText()
					System.out.println(sRefundResponse1)
					sFinalRefundAmountSAT = (sRefundResponse1.substring(24,35)).trim()
					//					String sRefundAmnt4 = sRefundAmnt3.trim()
					System.out.println("Refund Amount displayed in the final screen of SAT Window after Partial Refund  " + sFinalRefundAmountSAT )//1109.00

					if(sFinalRefundAmount == sFinalRefundAmountSAT)
					{
						System.out.println(" Total fare before Refund is  "+sFareAmount+" and after Partial refund Refundable amount is  "+sFinalRefundAmountSAT+" is correct ***************")
						Windows.click(findWindowsObject('SmartPoint/SAT/PartialRefund/ButtonConfirmPartialRefund'))
						WebUI.delay(5)
						Windows.click(findWindowsObject('SmartPoint/SAT/PartialRefund/ButtonFinalOK'))
						WebUI.delay(3)
						Windows.click(findWindowsObject('SmartPoint/SAT/FullRefund/ButtonCancel'))
						Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('IR', Keys.ENTER))
						Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('*HTE', Keys.ENTER))
					}
					else
					{
						System.out.println('Mismatch in the refund balance')
					}
				}
			}
		}
		else
		{
			System.out.println('Verify the *HTE response in Temrinal window')
		}

	}

	public TicketedPNRstatus() {
		/* ********************************************************************************************************************
		 FUNCTION    : TicketedPNRstatus()
		 AUTHOR      : Samundeswari K
		 DESCRIPTION : Function to Verify Existing Ticketed PNR Status
		 DATE        : 10'May22
		 EXAMPLE     : TicketedPNRstatus()
		 *********************************************************************************************************************/
		String sTicketedPNRstatus // Object to Verify Existing Ticketed PNR Status

		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/Document'), Keys.chord('*HTE', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/Document'), Keys.chord(Keys.ALT, 'c'))
		String my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		System.out.println(my_clipboard)
		sTicketedPNRstatus =  my_clipboard
		if (sTicketedPNRstatus.contains('OPEN'))
		{
			System.out.println('Match found for Open')
		}

		System.out.println(sTicketedPNRstatus)

		int index2=sTicketedPNRstatus.indexOf("TKT: ");//TKT: 220 9520 566804-805
		System.out.println(index2);
		int index3=index2+5
		int index4=index3+19
		TKTNO = sTicketedPNRstatus.substring(index3,index4)
		System.out.println(TKTNO)//220 9520 566804-805
		String TKTNO1 = TKTNO.replaceAll("\\s", "")//2209520566804-805
		TKTNO1 = TKTNO1.replaceAll("-","")
		System.out.println(TKTNO1)//2209520566804805
		boolean result = TKTNO1.matches("[0-9]{13}")
		if (result == true)
		{
			System.out.println("Ticket number Contains 13 Digits and Ticket number is "+TKTNO)
		}
		else
		{
			System.out.println("Ticket number not Contains 13 Digits and Ticket number is "+TKTNO)
			assert false
		}
		return TKTNO
	}

	public ManualEvenExchange() {
		/* ********************************************************************************************************************
		 FUNCTION    : ManualEvenExchange()
		 AUTHOR      : Samundeswari K
		 DESCRIPTION : Function to Manual Even Exchange operation
		 DATE        : 10'May22
		 EXAMPLE     : ManualEvenExchange()
		 *********************************************************************************************************************/
		String docs  // Object to store data from Clip_board
		String exchangeTicket // Object to store data from Clip_board having exchange ticket
		String openTicket	// Object to store data from Clip_board having exchange ticket


		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/Document'), Keys.chord('TKPFEX'+TKTNO,Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/Document'), Keys.chord(Keys.UP,))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/Document'), Keys.chord(Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/Document'), Keys.chord(Keys.UP,))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/Document'), Keys.chord(Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/Document'), Keys.chord(Keys.TAB,))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/Document'), Keys.chord('S',Keys.ENTER))
		//Verify Supporting documents

		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/Document'), Keys.chord(Keys.ALT, 'c'))
		String my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		System.out.println(my_clipboard)
		docs =  my_clipboard
		if (docs.contains('SUPPORTING DOCUMENTS GENERATED'))
		{
			System.out.println('Match found for Open : ' + docs)
		}
		Windows.setText(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), '*' + GlobalVariable.PNR)
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/Document'), Keys.chord('R.P+ER',Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/Document'), Keys.chord('*HTE',Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/Document'), Keys.chord('*TE001',Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/Document'), Keys.chord(Keys.ALT, 'c'))
		String my_clipboarda = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		System.out.println(my_clipboarda)
		exchangeTicket =  my_clipboarda
		if (exchangeTicket.contains('EXCH'))
		{
			System.out.println('Match found for EXCH')
		}
		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/Document'), Keys.chord('*HTE',Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/Document'), Keys.chord('*TE002',Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/Document'), Keys.chord(Keys.ALT, 'c'))
		String my_clipboards = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		System.out.println(my_clipboards)
		openTicket =  my_clipboards
		if (openTicket.contains('OPEN'))
		{
			System.out.println('Match found for Open')
		}
	}

	@Keyword
	public checkVoidCheckBox(String sPaxname,String sSegment)
	{
		/* *******************************************************************************************************************
		 Function	 : checkVoidCheckBox
		 AUTHOR      :  Nishanth.Hk
		 Description : Function to check Void Check Box
		 Date		 : 10'May22
		 Example     : checkVoidCheckBox
		 *********************************************************************************************************************/
		int iNoOfPassenger//To store number of Passenger
		String[] aPassengerName//to sore passenger Name

		////**************************************Creating object to identify Passenger Name***********************************************
		WindowsTestObject no0 = new WindowsTestObject("objectName1")
		String xpath0 = '//Text[@AutomationId="TextBlock_2ec93186272ccdc"]'
		no0.setLocatorStrategy(LocatorStrategy.XPATH)
		no0.setLocator(xpath0)
		List<WebElement> foundElements1 = Windows.findElements(no0)
		iNoOfPassenger = foundElements1.size()
		System.out.println("^^^^^^^^^^ There are "+iNoOfPassenger+" ^^^^^^^^^^")

		aPassengerName = new String[iNoOfPassenger]
		for(int i = 0; i < iNoOfPassenger; i++)
		{

			aPassengerName[i]=foundElements1[i].getText()//Storing Passenger Names in str1 array
			System.out.println("^^^^^^^^^^ Passenger Name"+(i+1)+" is "+aPassengerName[i]+" ^^^^^^^^^^")//Printing Passenger Names

		}
		// //**************************************Creating object to identify Ticket Number***********************************************
		WindowsTestObject to1 = new WindowsTestObject("objectName1")
		String xpath1 = '//Text[@AutomationId="TextBlock_2ec93186272ce9e"]'
		to1.setLocatorStrategy(LocatorStrategy.XPATH)
		to1.setLocator(xpath1)
		List<WebElement> foundElements2 = Windows.findElements(to1)
		iNoOfTickets = foundElements2.size()
		System.out.println("^^^^^^^^^^ There are "+iNoOfTickets+" Tickets ^^^^^^^^^^ " )
		for(int i = 0; i < iNoOfTickets; i++)
		{

			aTicketNumber[i]=foundElements2[i].getText()
			System.out.println("^^^^^^^^^^ Passenger"+(i+1)+" Ticket Number is "+aTicketNumber[i]+" ^^^^^^^^^^")

		}

		Windows.sendKeys(findWindowsObject('SmartPoint/SAT/Void/CheckBox_Void'), Keys.chord( Keys.SPACE))

	}

	@Keyword
	public voidOperation()
	{
		/* *******************************************************************************************************************
		 Function	 : voidOperation
		 AUTHOR      :  Nishanth.Hk
		 Description : Function to perform void Operation
		 Date        : 10'May22
		 Example     : voidOperation
		 *********************************************************************************************************************/
		int      iNoResponse   //To store no of void Response present in screen
		String[] aVoidResponse //To store void Response present in screen

		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/Void/ButtonVoid'))
		WebUI.delay(3)
		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/Void/ButtonYesvoid'))
		WebUI.delay(5)
		WindowsTestObject sVoidResponse = new WindowsTestObject("objectName5")//Creating object to identify Response message for Void request
		String xpath5 = '//Text[@AutomationId="TextBlock_2ec9318626e979e"]'
		sVoidResponse.setLocatorStrategy(LocatorStrategy.XPATH)
		sVoidResponse.setLocator(xpath5)
		WebUI.delay(5)
		List<WebElement> foundElements5 = Windows.findElements(sVoidResponse)
		iNoResponse = foundElements5.size()
		System.out.println(iNoResponse)
		aVoidResponse = new String[iNoResponse]
		for(int i = 0; i < iNoResponse; i++)
		{

			aVoidResponse[i]=foundElements5[i].getText()

		}

		int j =0
		for(int i = 0; i < iNoResponse ; i++)
		{
			if(aVoidResponse[i].contains('TRV/'+aTicketNumber[j]))
			{
				System.out.println('TRV/'+aTicketNumber[j]+' Present in window')
			}
			else
			{
				System.out.println('TRV/'+aTicketNumber[j]+' Not Present in window')
				assert false
			}

			i++

			if(aVoidResponse[i].contains('TICKET NO.'+aTicketNumber[j]+'CHANGED TO VOID STATUS'))
			{
				System.out.println("TICKET NO."+" "+aTicketNumber[j]+" CHANGED TO VOID STATUS"+" Present in window")
			}
			else
			{
				System.out.println("TICKET NO."+" "+aTicketNumber[j]+" CHANGED TO VOID STATUS"+" Not present in window")
				assert false
			}
			j=j+1

		}

		Windows.doubleClick(findWindowsObject('SmartPoint/SAT/Void/ButtonOkVoid'))
	}

	@Keyword
	public validateTicketDetailsRemovedFromSATScreen()
	{
		/* *******************************************************************************************************************
		 Function    : validateTicketDetailsRemovedFromSATScreen
		 AUTHOR      :  Nishanth.Hk
		 Description : Function to validate Ticket Details Removed From SATScreen
		 Date		 : 10'May22
		 Example 	 : validateTicketDetailsRemovedFromSATScreen
		 *********************************************************************************************************************/
		WebUI.delay(5)
		WindowsTestObject to7 = new WindowsTestObject("objectName7")//Object to identify Ticket Number on SAT screen
		String xpath7 = '//Text[@AutomationId="TextBlock_2ec93186272ce9e"]'
		to7.setLocatorStrategy(LocatorStrategy.XPATH)
		to7.setLocator(xpath7)
		List<WebElement> foundElements7 = Windows.findElements(to7)
		int t7 = foundElements7.size()
		System.out.println(t7)
		if(t7 == 0)
		{
			System.out.println('^^^^^^^^^^ No tickets Present in SAT screen ^^^^^^^^^^')
		}
		else
		{
			System.out.println('^^^^^^^^^^ '+t7+' tickets Present in SAT screen ^^^^^^^^^^ ')
			assert false
		}
	}

	@Keyword
	public closeSAT()
	{
		/* *******************************************************************************************************************
		 Function    : closeSAT()
		 AUTHOR      :  Nishanth.Hk
		 Description : Function to close SAT screen
		 Date		 : 10'May22
		 Example 	 : closeSAT()
		 *********************************************************************************************************************/
		Windows.click(findWindowsObject('SmartPoint/SAT/Void/ButtonSatCloseVoid'))
	}

	public validateTicketsStatusForVoid()
	{
		/* *******************************************************************************************************************
		 Function	 : validateTicketsStatusForVoid()
		 AUTHOR      :  Nishanth.Hk
		 Description : Function to validate the tickets whether ticket is in Void status or not
		 Date		 : 12'Apr22
		 Example     : validateTicketsStatusForVoid()
		 *********************************************************************************************************************/
		boolean bTktStatusVoid
		//String my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('IR', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('*HTE', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.ALT, 'c'))
		String my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		if(my_clipboard.contains('TKT:'))
		{
			System.out.println('Contains: ' + my_clipboard.contains('VOID'))
			bTktStatusVoid = my_clipboard.contains('VOID')

			if (bTktStatusVoid == true) {
				System.out.println('^^^^^^^^^^ Ticket in Void status ^^^^^^^^^^ ')
			}
			else {
				System.out.println('^^^^^^^^^^ Ticket not in Void status ^^^^^^^^^^')
				assert false
			}
		}
		else
		{
			for(int i = 1; i <= iNoOfTickets; i++)
			{
				Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('*TE00'+i, Keys.ENTER))
				Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.ALT, 'c'))
				my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
				System.out.println('Contains: ' + my_clipboard.contains('VOID'))
				bTktStatusVoid = my_clipboard.contains('VOID')
				if (bTktStatusVoid == true) {
					System.out.println('^^^^^^^^^^ *TE00'+i+' in Void status ^^^^^^^^^^ ')
				}
				else {
					System.out.println('^^^^^^^^^^ *TE00'+i+' Not in Void status ^^^^^^^^^^')
					assert false
				}
			}
		}

	}

	public void performManualFullRefundOperation(String sDate,String sCustomer)
	{
		/* *******************************************************************************************************************
		 Function	 : performManualFullRefundOperation
		 AUTHOR      :  Nishanth.Hk
		 Description : Function to perform Manual Full Refund Operation
		 Date		 : 12'Apr22
		 Example 	 : performManualFullRefundOperation
		 *********************************************************************************************************************/
		boolean bTktStatus1
		boolean bTktStatus2
		String sFare
		String[] aTickets

		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('*HTE', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.ALT, 'c'))
		String my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		if(my_clipboard.contains('TKT:'))
		{

			System.out.println('^^^^^^^^^^ No of tickets is 1 ^^^^^^^^^^ ')
			iNoOfTickets = 1

		}
		else {
			//			System.out.println(my_clipboard)
			int sStringLength =my_clipboard.length()
			//			System.out.println(sStringLength)
			int sStringLengthNew = (sStringLength)-(113)
			//			System.out.println(sStringLengthNew)
			String sStrchar = my_clipboard.substring(sStringLengthNew)
			//			System.out.println(sStrchar)//>*TE002·   VENKAT/CV        0149521274625 END OF LIST  >
			String sStrchar1 = my_clipboard.substring(sStringLengthNew,(sStringLengthNew+7))
			//			System.out.println(strchar1)//>*TE002
			//			String strchar2 = my_clipboard.contains('*TE00')
			//			if (strchar2 == true) {
			//				System.out.println('^^^^^^^^^^ Text *TE00 Present in Terminal window ^^^^^^^^^^ ')
			//			}
			//			else{
			//				System.out.println('^^^^^^^^^^ Text *TE00 not present in Terminal window.The text present is '+sStrchar1+' ^^^^^^^^^^')
			//				assert false
			//			}
			//			iNoOfTickets = my_clipboard.substring((sStringLengthNew+6),(sStringLengthNew+7))
			String sNoOfTickets = sStrchar1.substring(6)
			iNoOfTickets = Integer.parseInt(sNoOfTickets)

			System.out.println('^^^^^^^^^^ No of tickets is '+ iNoOfTickets+' ^^^^^^^^^^')//No of tickets is 2
		}

		aTickets = new String[10]
		System.out.println(my_clipboard)
		for(int i = 1; i <= iNoOfTickets; i++)
		{
			if(iNoOfTickets == 1)
			{
				Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('*HTE', Keys.ENTER))
			}
			else
			{
				Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('*TE00'+i, Keys.ENTER))
			}
			Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.ALT, 'c'))
			my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
			int index2=my_clipboard.indexOf("TKT: ");//TKT: 220 9520 566804
			System.out.println(index2);
			int index3=index2+5
			int index4=index3+15
			String TKTNO = my_clipboard.substring(index3,index4)
			System.out.println(TKTNO)//220 9520 566804
			String TKTNO1 = TKTNO.replaceAll("\\s", "")//2209520566804
			TKTNO1 = TKTNO1.replaceAll("-","")
			System.out.println(TKTNO1)//2209520566804
			aTickets[i] = TKTNO1
			System.out.println(aTickets[i])
			//boolean result = TKTNO1.matches("[0-9]{13}")
			//if (result == true)
			bTktStatus1 = my_clipboard.contains('OPEN')
			if (bTktStatus1 == true)
			{
				System.out.println("Passanger"+(i)+" Ticket number is "+aTickets[i])
				System.out.println("Ticket in OPEN Status")
			}
			else
			{

				System.out.println("Passanger"+(i)+" Ticket number is "+aTickets[i])
				System.out.println("Ticket not in OPEN Status")
				assert false
			}
			index2=my_clipboard.indexOf("TOTAL ");//returns the index of index substring
			System.out.println(index2);
			index3=index2+11
			index4=index3+7
			int  index5=index3+10
			sFare = my_clipboard.substring(index3,index5)
			sFare = sFare.trim()
			System.out.println(sFare)//775.80

			Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('TRNE'+aTickets[i]+'/'+sDate, Keys.ENTER))
			Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.TAB))
			Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.TAB))
			Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('0.00'))
			Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.TAB))
			Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.TAB))
			Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.TAB))
			Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('Y', Keys.ENTER))
			Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.SHIFT+Keys.TAB))
			Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.ENTER))
			Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.SHIFT+Keys.TAB))
			Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('N', Keys.ENTER))
			Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.ALT, 'c'))
			//**********************************************************************************************************************
			switch (sCustomer) {
				case 'Edreams':
					System.out.println('Selected Edreams Customer' )
					my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
					System.out.println(my_clipboard)
					bTktStatus1 = my_clipboard.contains('AUTOMATED REFUND NOTICE ISSUED')
					bTktStatus2 = my_clipboard.contains('CASH REFUND AMOUNT '+sFare)
					System.out.println('CASH REFUND AMOUNT '+sFare)
					if ((bTktStatus1 == true) && (bTktStatus2 == true))
					{
						System.out.println('^^^^^^^^^ Text AUTOMATED REFUND NOTICE ISSUED Present in window^^^^^^^^^')
						System.out.println('^^^^^^^^^ Text CASH REFUND AMOUNT '+sFare+'Present in window^^^^^^^^^')
						System.out.println('^^^^^^^^^ Full Refund  is complted successfully ^^^^^^^^^')

					}
					else
					{

						System.out.println('^^^^^^^^^ Text AUTOMATED REFUND NOTICE ISSUED not Present ^^^^^^^^^')
						System.out.println('^^^^^^^^^ Text CASH REFUND AMOUNT not Present^^^^^^^^^')
						System.out.println('^^^^^^^^^ Full Refund Not complted ^^^^^^^^^')
						assert false
					}
					break;
				case 'Delta Vacations':
					my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
					System.out.println(my_clipboard)
					bTktStatus1 = my_clipboard.contains('REFUND COMPLETE')
					bTktStatus2 = my_clipboard.contains('CASH REFUND AMOUNT '+sFare)
					System.out.println('CASH REFUND AMOUNT '+sFare)
					if ((bTktStatus1 == true) && (bTktStatus2 == true))
					{
						System.out.println('^^^^^^^^^Text REFUND COMPLETE Present in window^^^^^^^^^')
						System.out.println('^^^^^^^^^ Text CASH REFUND AMOUNT '+sFare+'Present in window^^^^^^^^^')
						System.out.println('^^^^^^^^^ Full Refund  is complted successfully ^^^^^^^^^')

					}
					else
					{
						System.out.println('^^^^^^^^^ REFUND COMPLETE not Present^^^^^^^^^')
						System.out.println('^^^^^^^^^ CASH REFUND AMOUNT not Present^^^^^^^^^')
						System.out.println('^^^^^^^^^ Full Refund Not complted ^^^^^^^^^')
						assert false
					}
					break;
				default:
					System.out.println('Select Valid Customer' )
					break;
			}

			Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('IR', Keys.ENTER))
		}
	}
	public static CancelTheItenary() {
		/* *******************************************************************************************************************
		 Function	 : CancelTheItenary
		 AUTHOR      : Nishanth.Hk
		 Description : Function to perform cancel the itenary and cancel the Fare
		 Date		 : 31'May22
		 Example 	 : CancelTheItenary
		 *********************************************************************************************************************/
		String sMy_clipboard
		boolean bIsTextPresentInWindow
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('XI', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('R.P+ER', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('ER', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('IR', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('*FF', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord(Keys.ALT, 'c'))
		sMy_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		bIsTextPresentInWindow = sMy_clipboard.contains('FILED FARES DO NOT EXIST')
		if(bIsTextPresentInWindow == true) {
			System.out.println('^^^^^^Text FILED FARES DO NOT EXIST is present in the Window ^^^^^^')
		}
		else {
			System.out.println('^^^^^^Text FILED FARES DO NOT EXIST is not present in the Window ^^^^^^')
			assert false
		}
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('*TD', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord(Keys.ALT, 'c'))
		sMy_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		bIsTextPresentInWindow = sMy_clipboard.contains('NO TICKETING DATA EXISTS')
		if(bIsTextPresentInWindow == true) {
			System.out.println('^^^^^^Text NO TICKETING DATA EXISTS is present in the Window ^^^^^^')
		}
		else {
			System.out.println('^^^^^^Text NO TICKETING DATA EXISTS not present in the Window ^^^^^^')
			assert false
		}
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('I', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('I', Keys.ENTER))

	}

	public void validateTicketsStatusForRFND()
	{
		/* *******************************************************************************************************************
		 Function    : validateTicketsStatusForRFND
		 AUTHOR      :  Nishanth.Hk
		 Description : Function to validate Tickets Status For RFND
		 Date		 : 12'Apr22
		 Example 	 : validateTicketsStatusForRFND
		 *********************************************************************************************************************/
		boolean bTktStatusRFND
		//String my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('IR', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('*HTE', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.ALT, 'c'))
		String my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		if(my_clipboard.contains('TKT:'))
		{
			System.out.println('Contains: ' + my_clipboard.contains('RFND'))
			bTktStatusRFND = my_clipboard.contains('RFND')

			if (bTktStatusRFND == true) {
				System.out.println('^^^^^^^^^ Ticket in RFND status ^^^^^^^^^')
			}
			else {
				System.out.println('^^^^^^^^^ Ticket not in RFND status ^^^^^^^^^')
				assert false

			}
		}
		else
		{
			for(int i = 1; i <= iNoOfTickets; i++)
			{
				Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('*TE00'+i, Keys.ENTER))
				Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.ALT, 'c'))
				my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
				System.out.println('Contains: ' + my_clipboard.contains('RFND'))
				bTktStatusRFND = my_clipboard.contains('RFND')
				if (bTktStatusRFND == true) {
					System.out.println('^^^^^^^^^ *TE00'+i+' in RFND status ^^^^^^^^^')
				}
				else {
					System.out.println('^^^^^^^^^ *TE00'+i+' Not in RFND status ^^^^^^^^^')
					assert false

				}
			}
		}

	}
}
