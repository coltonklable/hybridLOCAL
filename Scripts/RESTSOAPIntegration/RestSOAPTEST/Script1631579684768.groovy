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
import groovy.json.JsonSlurper as JsonSlurper
import java.io.File as File

OauthValueResponse = WS.sendRequest(findTestObject('null', [('variable') : '']))

println('Ram' /*
				 * WS.verifyElementPropertyValue(OauthValueResponse, 'access_token', '')
				 */ )

GlobalVariable.token = WS.getElementPropertyValue(OauthValueResponse, 'access_token')

println(GlobalVariable.token)

ShopResponse = WS.sendRequest(findTestObject('RESTSOAPAPItest/Search Request RoundTrip GDS_old', [('Version') : GlobalVariable.Version
            , ('guid') : GlobalVariable.guid, ('XAUTH_TRAVELPORT_ACCESSGROUP_1G') : GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G
            , ('token') : GlobalVariable.token]))

CatalogOfferingsQueryRequest = WS.getElementPropertyValue(ShopResponse, 'CatalogOfferingsQueryRequest')

data = ShopResponse.getResponseText()

println(data)

f = new File('C:/Users/priya.vadakasi/Desktop/SVT/POC_REST_SOAP/log.txt')

JsonSlurper slurper = new JsonSlurper()

Map parsedJson = slurper.parseText(data)

/* f.append(data +"Help") */
datais = parsedJson.get('CatalogOfferingsResponse').get('CatalogOfferings').get('CatalogOffering').get(0).get('id')

System.out.println(datais)

f.append(datais)

f = new File('C:/Users/priya.vadakasi/Desktop/SVT/POC_REST_SOAP/log.txt')

//WS.sendRequest(findTestObject('SOAP Service (2)/AirLowFareSearchBinding/service'))
WS.sendRequest(findTestObject('SOAP Service (2)/AirPriceBinding/service'))

