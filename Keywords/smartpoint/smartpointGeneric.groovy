package smartpoint

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
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
import com.sun.java.swing.plaf.windows.resources.windows_es
import com.kms.katalon.core.testobject.WindowsTestObject as WindowsTestObject
import com.kms.katalon.core.testobject.WindowsTestObject.LocatorStrategy as LocatorStrategy
import org.openqa.selenium.WebElement
import groovy.ui.view.WindowsDefaults
import org.openqa.selenium.Keys as Keys
import internal.GlobalVariable
import java.awt.Toolkit as Toolkit
import java.awt.datatransfer.DataFlavor as DataFlavor
import cAPI.*

public class smartpointGeneric {

	@Keyword
	static sWindowName
	static sCore
	static sHideGalToggle
	public static Integer   sNoOfTickets
	public static LaunchSmartpoint() {
		Extbase.setNoProxy()
		Windows.startApplicationWithTitle('C:\\fp\\swdir\\viewpoint.exe', 'Login')
		if (Windows.verifyElementPresent(findWindowsObject('Object Repository/SmartPoint/Generic/WindowTitle/Start Galileo Desktop'), 0, FailureHandling.OPTIONAL)) {
			Windows.click(findWindowsObject('Object Repository/SmartPoint/Generic/WindowTitle/Start Galileo Desktop'))
		}
		Windows.delay(90)
		Windows.switchToWindowTitle('Application Window 1',FailureHandling.OPTIONAL)
		Windows.switchToWindowTitle('Galileo Desktop - [', FailureHandling.OPTIONAL)
		CloseAdditionalWindow()
		sWindowName = Windows.getDriver().title
		System.out.println('Window Title' + sWindowName )
		System.out.println(sWindowName)
		Windows.click(findWindowsObject('SmartPoint/Generic/MenuList/Menu/Application'), FailureHandling.OPTIONAL)
		Windows.delay(3)
		Windows.doubleClick(findWindowsObject('SmartPoint/Generic/MenuList/ApplicationSettings/ApplicationSettingsName'), FailureHandling.OPTIONAL)
		sHideGalToggle = Windows.getAttribute(findWindowsObject('SmartPoint/Generic/MenuList/ApplicationSettings/HideGalileoDesktop'),
				'Toggle.ToggleState', FailureHandling.OPTIONAL)
		System.out.println(sHideGalToggle)

		if (sHideGalToggle == '1') {
			System.out.println('Hide Galileo Destkop is ON ')
		} else if (sHideGalToggle == '0') {
			System.out.println('Hide Galileo Destkop is OFF')
			Windows.click(findWindowsObject('SmartPoint/Generic/MenuList/ApplicationSettings/HideGalileoDesktop'), FailureHandling.OPTIONAL)
			Windows.click(findWindowsObject('SmartPoint/Generic/MenuList/ApplicationSettings/ButtonHideGDOK'), FailureHandling.OPTIONAL)
			Windows.click(findWindowsObject('SmartPoint/Generic/MenuList/ApplicationSettings/ButtonSave'), FailureHandling.STOP_ON_FAILURE)
			Windows.click(findWindowsObject('SmartPoint/Generic/MenuList/ApplicationSettings/ButtonYes'), FailureHandling.STOP_ON_FAILURE)
			Windows.delay(90)
		}
		sWindowName = Windows.getDriver().title
		System.out.println('Window Title')
		System.out.println(sWindowName)
		if (sWindowName.contains('Galileo Desktop - [') == true) {
			System.out.println('*******OH!!! still am in Galileo Desktop I need a Switch in title*******')
			Windows.switchToWindowTitle('Application Window 1', FailureHandling.OPTIONAL)
		}
		else if (sWindowName.contains('Application Window 1') == true) {
			System.out.println('*******Am already in Application Window and i dont need a Switch in title*******')
			Windows.click(findWindowsObject('SmartPoint/Generic/MenuList/ApplicationSettings/ButtonCancel'), FailureHandling.STOP_ON_FAILURE)
		}

		//Windows.switchToWindowTitle('Galileo Desktop - [Booking File - No Names]')
		//Windows.switchToWindowTitle('Galileo Desktop - [PNR - No Names]')
		//Windows.switchToWindowTitle('Galileo Desktop - [PNR - VENKAT]')
		//Windows.switchToWindowTitle('Galileo Desktop - [Booking File - TEST]')
		//Windows.switchToWindowTitle('Application Window 1')
	}
	@Keyword
	public static LoginSmartpoint() {
		if (Windows.verifyElementPresent(findWindowsObject('SmartPoint/Generic/SignON/WinSignOn'), 0, FailureHandling.OPTIONAL)) {
			SignOn()
		}
		else if  (Windows.verifyElementPresent(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), 0, FailureHandling.OPTIONAL)) {
			Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('SOF', Keys.ENTER))
			if (Windows.verifyElementPresent(findWindowsObject('SmartPoint/Generic/SignON/WinSignOn'), 0, FailureHandling.OPTIONAL)) {
				SignOn()
			}
			else {
				Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord(Keys.ALT, 'c'))
				String my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
				System.out.println(my_clipboard)
				if (my_clipboard.contains('SIGN-OFF COMPLETE')) {
					Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord('I', Keys.ENTER))
					SignOn()
				}
			}
		}
	}

	public static SignOn() {
		Windows.setText(findWindowsObject('SmartPoint/Generic/SignON/TxtSignonGal'), 'E35599')
		//	encrypted pass for UAT
		Windows.setEncryptedText(findWindowsObject('SmartPoint/Generic/SignON/TxtPassGal'),'gIBU3RwkBkkytQXHeIOTtg==')
		//encrypted pass for QA
		//		Windows.setEncryptedText(findWindowsObject('SmartPoint/Generic/SignON/TxtPassGal'),'gIBU3RwkBkkytQXHeIOTtg==')
		Windows.click(findWindowsObject('SmartPoint/Generic/SignON/ButtonOK'))
		CloseAdditionalWindow()

	}

	public static CloseAdditionalWindow() {
		if (Windows.verifyElementPresent(findWindowsObject('SmartPoint/Generic/SignON/WinSignOn'), 2, FailureHandling.OPTIONAL)) {
			Windows.click(findWindowsObject('SmartPoint/Generic/SignON/ButtonCancel'))
		}
		if (Windows.verifyElementPresent(findWindowsObject('SmartPoint/Generic/SignON/WinEmail'), 3, FailureHandling.OPTIONAL)) {
			Windows.click(findWindowsObject('SmartPoint/Generic/SignON/ButtonCancelMail'))
			System.out.println('I closed email window')
		}
		if (Windows.verifyElementPresent(findWindowsObject('SmartPoint/Generic/SignON/WinWelcomeSP'), 5, FailureHandling.OPTIONAL)) {
			Windows.click(findWindowsObject('SmartPoint/Generic/SignON/ButtonCloseFlex'))
			System.out.println('I closed flex window')
			Windows.delay(5)
		}
		if (Windows.verifyElementPresent(findWindowsObject('SmartPoint/Generic/SignON/WhatsNew'), 5, FailureHandling.OPTIONAL)) {

			Windows.click(findWindowsObject('SmartPoint/Generic/SignON/ButtonCloseWhatsNew'))
			System.out.println('I closed Whats new window')
		}
	}

	public static ChangeCurrency(String sCtry) {
		StringBuilder sBuCtry  = new StringBuilder()
				.append('HMCT-')
				.append(sCtry)
				.append('/I')
		System.out.println('Country details :' +sBuCtry)
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord(sBuCtry, Keys.ENTER))
	}

	public static EmulatePCC(String sPcc) {
		//				StringBuilder sBuPcc  = new StringBuilder()
		StringBuffer sBfPcc = new StringBuffer()
				.append('SEM/')
				.append(sPcc)
				.append('/AG')
		System.out.println('Country details :' +sBfPcc)
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord(sBfPcc, Keys.ENTER))

	}
	public static RetrivePNR() {
		String sHTEBUTTON
		//		StringBuffer sBfPNR = new StringBuffer()
		//				.append(sPNR)
		//		System.out.println('Country details :' +sBfPNR)
		//		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord(sBfPNR, Keys.ENTER))
		println "Retrieve the ticketed PNR"
		Windows.setText(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), '*' + GlobalVariable.PNR)
		Windows.sendKeys(findWindowsObject('SmartPoint/Generic/WindowTitle/Document'), Keys.chord(Keys.ENTER))

		//***********************added by Nishanth********************************************************//
		sHTEBUTTON = Windows.getAttribute(findWindowsObject('SmartPoint/SAT/ButtonsPnrViewer/ButtonHTE'),'IsOffscreen')
		System.out.println(sHTEBUTTON)//False
		if(sHTEBUTTON == 'False')
		{
			System.out.println('Button *HTE Present in PNR Viewer')

		}
		else
		{
			System.out.println('Button *HTE not Present in PNR Viewer')
			assert false
		}
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord('*HTE', Keys.ENTER))
		Windows.sendKeys(findWindowsObject('Object Repository/SmartPoint/SAT/TerminalWindow'), Keys.chord(Keys.ALT, 'c'))
		String my_clipboard = Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor)
		if(my_clipboard.contains('TKT:'))
		{

			sNoOfTickets = 1
			System.out.println('^^^^^^^^^^ No of tickets is '+sNoOfTickets+' ^^^^^^^^^^ ')


		}
		else {
			int strlen =my_clipboard.length()
			int strlnnew = (strlen)-(113)
			String strchar = my_clipboard.substring(strlnnew)
			String strchar1 = my_clipboard.substring(strlnnew,(strlnnew+7))
			String strchar2 = strchar1.substring(6)
			sNoOfTickets = Integer.parseInt(strchar2)
			System.out.println('^^^^^^^^^^ No of tickets is '+ sNoOfTickets+' ^^^^^^^^^^')//No of tickets is 2
		}

	}


	public CloseSP() {
		if (Windows.verifyElementPresent(findWindowsObject('SmartPoint/Generic/SignON/ButtonCloseSP'), 5, FailureHandling.OPTIONAL)) {
			Windows.click(findWindowsObject('SmartPoint/Generic/SignON/ButtonCloseSP'))
			Windows.click(findWindowsObject('SmartPoint/Generic/SignON/YNPopupOnCloseSP'))
		}
	}
}
