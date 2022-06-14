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
import com.kms.katalon.core.webservice.verification.WSResponseManager as WSResponseManager
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import travelport.CASTLE.*
import groovy.json.JsonSlurper as JsonSlurper
import groovy.util.XmlSlurper as XmlSlurper
import groovy.util.XmlParser as XmlParser
import java.io.*
import java.util.*
import java.io.File as File
import com.kms.katalon.core.testobject.HttpBodyContent


GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyy-MM-dd', GlobalVariable.DateValue1)

println('StartDate' + GlobalVariable.StartDate)

AvailResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/OTA_AirAvailRQ', 
        [('StartDate') : GlobalVariable.StartDate, ('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Origin') : findTestData(
                'Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1), ('Password') : GlobalVariable.Password
            , ('Username') : GlobalVariable.Username]))

WS.verifyResponseStatusCode(AvailResponse, 200)

String xml1 = AvailResponse.responseText

println('xml1' + xml1)

def result1 = new XmlSlurper().parseText(xml1)

println('result1' + result1)

GlobalVariable.FlightNumber1 = result1.Body.ProviderTransactionResponse.RSP.OTA_AirAvailRS.OriginDestinationInformation.OriginDestinationOptions.OriginDestinationOption.FlightSegment[0].@FlightNumber

println('FlightNumber1  ' + GlobalVariable.FlightNumber1)

