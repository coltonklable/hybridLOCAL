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

Response1 = WS.sendRequest(findTestObject('XMLSelect_Galileo_Smoke/XmlSelectSoap/CAPI XMLSelect_1G/Availability', [('Username') : GlobalVariable.Username
            , ('PCC') : GlobalVariable.PCC1, ('Profile') : GlobalVariable.Profile1, ('Provider') : GlobalVariable.Provider
            , ('Origin') : findTestData('Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1)
            , ('Carrier') : findTestData('Air_1G').getValue(4, 1), ('StartDate') : GlobalVariable.StartDate]))

WS.verifyResponseStatusCode(Response1, 200)

String xml1 = Response1.responseBodyContent

def result1 = new XmlSlurper().parseText(xml1)

println('result1' + result1)

GlobalVariable.Carrier = result1.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].AirV.text()

println('Carrier' + GlobalVariable.Carrier)

String FlightNumber = result1.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].FltNum.text()

if (FlightNumber.length() < 4) {
    FlightNumber = ('0' + FlightNumber)
}

GlobalVariable.FlightNumber1 = FlightNumber

GlobalVariable.StartDate1 = result1.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].StartDt.text()

GlobalVariable.Origin = result1.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].StartAirp.text()

GlobalVariable.Destination = result1.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].EndAirp.text()

String StartTime = result1.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].StartTm.text()

if (StartTime.length() < 4) {
    StartTime = ('0' + StartTime)
}

GlobalVariable.StartTime1 = StartTime

String EndTime = result1.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].EndTm.text()

if (EndTime.length() < 4) {
    EndTime = ('0' + EndTime)
}

GlobalVariable.EndTime1 = EndTime

Response2 = WS.sendRequest(findTestObject('XMLSelect_Galileo_Smoke/XmlSelectSoap/CAPI XMLSelect_1G/BeginSession', [('Username') : GlobalVariable.Username
            , ('PCC') : GlobalVariable.PCC1, ('Profile') : GlobalVariable.Profile1, ('Provider') : GlobalVariable.Provider
            , ('Origin') : findTestData('Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1)
            , ('Carrier') : findTestData('Air_1G').getValue(4, 1), ('StartDate') : GlobalVariable.StartDate]))

WS.verifyResponseStatusCode(Response2, 200)

String xml2 = Response2.responseBodyContent

def result2 = new XmlSlurper().parseText(xml2)

GlobalVariable.TokenNumber = result2.BeginSessionResult.text()

println('TokenNumber' + GlobalVariable.TokenNumber)

Response3 = WS.sendRequest(findTestObject('XMLSelect_Galileo_Smoke/XmlSelectSoap/CAPI XMLSelect_1G/PNRBFManagement_Create PNR', 
        [('Username') : GlobalVariable.Username, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('Carrier') : GlobalVariable.Carrier
            , ('FlightNumber') : GlobalVariable.FlightNumber1, ('Origin') : GlobalVariable.Origin, ('Destination') : GlobalVariable.Destination
            , ('StartDate') : GlobalVariable.StartDate1, ('StartTime') : GlobalVariable.StartTime1, ('EndTime') : GlobalVariable.EndTime1
            , ('TokenNumber') : GlobalVariable.TokenNumber, ('TicketDate') : GlobalVariable.TicketDate]))

WS.verifyResponseStatusCode(Response3, 200)

String xml3 = Response3.responseBodyContent

def result3 = new XmlSlurper().parseText(xml3)

GlobalVariable.RecordLoc = result3.SubmitXmlOnSessionResult.PNRBFManagement_49.EndTransaction.EndTransactResponse[0].RecLoc.text()

println('RecordLoc' + GlobalVariable.RecordLoc)

Response4 = WS.sendRequest(findTestObject('XMLSelect_Galileo_Smoke/XmlSelectSoap/CAPI XMLSelect_1G/PNRBFManagement_Add Received info', 
        [('Username') : GlobalVariable.Username, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber
            , ('RecordLoc') : GlobalVariable.RecordLoc]))

WS.verifyResponseStatusCode(Response4, 200)

Response5 = WS.sendRequest(findTestObject('XMLSelect_Galileo_Smoke/XmlSelectSoap/CAPI XMLSelect_1G/FareQuote', [('Username') : GlobalVariable.Username
            , ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response5, 200)

Response6 = WS.sendRequest(findTestObject('XMLSelect_Galileo_Smoke/XmlSelectSoap/CAPI XMLSelect_1G/PNRRetrieve1', [('Username') : GlobalVariable.Username
            , ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber
            , ('RecordLoc') : GlobalVariable.RecordLoc]))

WS.verifyResponseStatusCode(Response6, 200)

Response7 = WS.sendRequest(findTestObject('XMLSelect_Galileo_Smoke/XmlSelectSoap/CAPI XMLSelect_1G/PNRBFManagement_StorePrice', 
        [('Username') : GlobalVariable.Username, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('Carrier') : GlobalVariable.Carrier
            , ('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response7, 200)

Response8 = WS.sendRequest(findTestObject('XMLSelect_Galileo_Smoke/XmlSelectSoap/CAPI XMLSelect_1G/PNRRetrieve2', [('Username') : GlobalVariable.Username
            , ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber
            , ('RecordLoc') : GlobalVariable.RecordLoc]))

WS.verifyResponseStatusCode(Response8, 200)

Response9 = WS.sendRequest(findTestObject('XMLSelect_Galileo_Smoke/XmlSelectSoap/CAPI XMLSelect_1G/DocProdFareManipulation', 
        [('Username') : GlobalVariable.Username, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response9, 200)

Response10 = WS.sendRequest(findTestObject('XMLSelect_Galileo_Smoke/XmlSelectSoap/CAPI XMLSelect_1G/Endsession', [('Username') : GlobalVariable.Username
            , ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response10, 200)

