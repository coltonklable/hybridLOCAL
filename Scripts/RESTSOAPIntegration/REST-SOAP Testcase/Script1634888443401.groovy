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
import travelport.CASTLE.*
import groovy.json.JsonSlurper as JsonSlurper
import groovy.util.XmlSlurper as XmlSlurper
import java.io.*
import java.util.*
import java.io.File as File

response = WS.sendRequest(findTestObject('RESTSOAPAPItest/OpenShift Oauth PRP 86W0'))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'token_type', 'Bearer')

def slurper = new JsonSlurper()

def result = slurper.parseText(response.getResponseBodyContent())

def token = result.access_token

def token_type = result.token_type

def auth = (token_type + ' ') + token

GlobalVariable.token = auth

//println('From ' + From)
//println('To' + To)
response1 = WS.sendRequest(findTestObject('RESTSOAPAPItest/Search Request RoundTrip GDS', [('Version') : GlobalVariable.Version
            , ('guid') : GlobalVariable.guid, ('XAUTH_TRAVELPORT_ACCESSGROUP_1G') : GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G
            , ('token') : GlobalVariable.token]))

WS.verifyResponseStatusCode(response1, 200)

result = slurper.parseText(response1.getResponseBodyContent())

GlobalVariable.Carrier = result.CatalogOfferingsResponse.ReferenceList[0].Flight[0].carrier

GlobalVariable.FlightNumber = result.CatalogOfferingsResponse.ReferenceList[0].Flight[0].number

GlobalVariable.Origin = result.CatalogOfferingsResponse.ReferenceList[0].Flight[0].Departure.location

GlobalVariable.Destination = result.CatalogOfferingsResponse.ReferenceList[0].Flight[0].Arrival.location

GlobalVariable.DepartureTime = result.CatalogOfferingsResponse.ReferenceList[0].Flight[0].Departure.date

//GlobalVariable.DepartureTime1 = result.CatalogOfferingsResponse.ReferenceList[0].Flight[0].Departure.time
GlobalVariable.ArrivalTime = result.CatalogOfferingsResponse.ReferenceList[0].Flight[0].Arrival.date

//GlobalVariable.ArrivalTime1 = result.CatalogOfferingsResponse.ReferenceList[0].Flight[0].Arrival.time
GlobalVariable.Distance = result.CatalogOfferingsResponse.ReferenceList[0].Flight[0].distance

GlobalVariable.Equipment = result.CatalogOfferingsResponse.ReferenceList[0].Flight[0].equipment

response2 = WS.sendRequest(findTestObject('RESTSOAPAPItest/AirPrice Request', [('Carrier') : GlobalVariable.Carrier, ('FlightNumber') : GlobalVariable.FlightNumber
            , ('Origin') : GlobalVariable.Origin, ('Destination') : GlobalVariable.Destination, ('DepartureTime') : GlobalVariable.DepartureTime
            , ('ArrivalTime') : GlobalVariable.ArrivalTime, ('Distance') : GlobalVariable.Distance, ('Equipment') : GlobalVariable.Equipment]))

WS.verifyResponseStatusCode(response2, 200)

String xml1 = response2.responseBodyContent

//def result1 = new XmlSlurper().parseText(xml1)

//println('result1' + result1)

//def Segment = xml1.AirItinerary[0].AirSegment

int index = xml1.indexOf('<air:AirSegment')

println('index' + index)

int index1 = xml1.indexOf('</air:AirSegment>')

println('index1' + index1)

String segment1 = xml1.substring(index, index1 + 17)

println('Segment' + segment1)

int index2 = xml1.indexOf('<air:AirPricingSolution')

println('index2' + index2)

int index3 = xml1.indexOf('</air:AirPricingSolution>')

println('index3' + index3)

String PricingSolution = xml1.substring(index2, index3 + 25)

println('PricingSolution' + PricingSolution)

String PricingSolution_Split = PricingSolution.split('<air:AirSegmentRef')

println('PricingSolution_Split[0]' + PricingSolution_Split[0])

println('PricingSolution_Split[1]' + PricingSolution_Split[1])

String Pricing = PricingSolution_Split[0] + segment1

String PricingInfo = PricingSolution.substring(Airpricing.indexOf('<air:AirPricingInfo'))

GlobalVariable.PricingSol = Pricing + PricingInfo

println('Final PricingSolution' + GlobalVariable.PricingSol)




