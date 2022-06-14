import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject

GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue1)

Response1 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/Availability', 
        [('Profile') : GlobalVariable.Profile1, ('Carrier') : findTestData('Air_1G').getValue(4, 1), ('Origin') : findTestData(
                'Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1), ('Username') : GlobalVariable.Username
            , ('StartDate') : GlobalVariable.StartDate]))

WS.verifyResponseStatusCode(Response1, 200)

String xml1 = Response1.responseBodyContent

def result = new XmlSlurper().parseText(xml1)

println(WS.getElementText(Response1, 'SubmitXmlResponse.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt.AirV'))

println('AirV' + result.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt.AirV)

println('Total Count Of AirV in response ' + WS.getElementsCount(Response1, 'SubmitXmlResponse.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt.AirV'))

println('AirV Values are ' + WS.getElementPropertyValue(Response1, 'SubmitXmlResponse.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt.AirV'))

WS.verifyElementPropertyValue(Response1, 'SubmitXmlResponse.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].AirV', 'LH')

