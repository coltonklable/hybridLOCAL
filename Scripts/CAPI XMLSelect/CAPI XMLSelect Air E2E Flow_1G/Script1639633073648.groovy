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
import com.kms.katalon.core.testobject.ResponseObject
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

GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue1)

GlobalVariable.EndDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue2)

Response1 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/Availability', 
        [('Profile') : GlobalVariable.Profile1, ('Carrier') : findTestData('Air_1G').getValue(4, 1), ('Origin') : findTestData(
                'Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1), ('Username') : GlobalVariable.Username
            , ('StartDate') : GlobalVariable.StartDate]))

WS.verifyResponseStatusCode(Response1, 200)

String xml1 = Response1.responseBodyContent

def result1 = new XmlSlurper().parseText(xml1)

println('result1' + result1)

println('Total Count Of AirV in response ' + WS.getElementsCount(Response1, 'SubmitXmlResponse.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt.AirV'))

WS.verifyElementPropertyValue(Response1, 'SubmitXmlResponse.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].AirV', 
    findTestData('Air_1G').getValue(4, 1))

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

Response2 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/AirAvailabilityByTariff', 
        [('Profile') : GlobalVariable.Profile1, ('Carrier') : findTestData('Air_1G').getValue(4, 1), ('Origin') : findTestData(
                'Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1), ('Username') : GlobalVariable.Username
            , ('StartDate') : GlobalVariable.StartDate]))

WS.verifyResponseStatusCode(Response2, 200)

Response3 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/AirAvailabilityWithCabin', 
        [('Profile') : GlobalVariable.Profile1, ('Carrier') : findTestData('Air_1G').getValue(4, 1), ('Origin') : findTestData(
                'Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1), ('Username') : GlobalVariable.Username
            , ('StartDate') : GlobalVariable.StartDate, ('EndDate') : GlobalVariable.EndDate]))

WS.verifyResponseStatusCode(Response3, 200)

Response4 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/BeginSession', 
        [('Profile') : GlobalVariable.Profile1]))

WS.verifyResponseStatusCode(Response4, 200)

String xml2 = Response4.responseBodyContent

def result2 = new XmlSlurper().parseText(xml2)

GlobalVariable.TokenNumber = result2.BeginSessionResult.text()

println('TokenNumber' + GlobalVariable.TokenNumber)

Response5 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/PNRBFManagement', 
        [('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('Carrier') : GlobalVariable.Carrier, ('FlightNumber') : GlobalVariable.FlightNumber1
            , ('Origin') : GlobalVariable.Origin, ('Destination') : GlobalVariable.Destination, ('StartDate') : GlobalVariable.StartDate1
            , ('StartTime') : GlobalVariable.StartTime1, ('EndTime') : GlobalVariable.EndTime1, ('TokenNumber') : GlobalVariable.TokenNumber
            , ('TicketDate') : GlobalVariable.TicketDate]))

WS.verifyResponseStatusCode(Response5, 200)

String xml3 = Response5.responseBodyContent

def result3 = new XmlSlurper().parseText(xml3)

GlobalVariable.RecordLoc = result3.SubmitXmlOnSessionResult.PNRBFManagement_49.EndTransaction.EndTransactResponse[0].RecLoc.text()

println('RecordLoc' + GlobalVariable.RecordLoc)

Response6 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/PNRBFManagement_PNRRetrieve', 
        [('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber
            , ('RecordLoc') : GlobalVariable.RecordLoc]))

WS.verifyResponseStatusCode(Response6, 200)

Response7 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteBestBuyCompare', 
        [('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response7, 200)

Response8 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteBestBuyRegardless', 
        [('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response8, 200)

Response9 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteDollarSaverInt', 
        [('Carrier') : GlobalVariable.Carrier, ('Origin') : GlobalVariable.Origin, ('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response9, 200)

Response10 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteDollarSaver', 
        [('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response10, 200)

Response11 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteStandardInt', 
        [('TokenNumber') : GlobalVariable.TokenNumber, ('Provider') : GlobalVariable.Provider, ('PCC') : GlobalVariable.PCC1]))

WS.verifyResponseStatusCode(Response11, 200)

Response12 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteCalendarDates', 
        [('Carrier') : GlobalVariable.Carrier, ('Origin') : GlobalVariable.Origin, ('Destination') : GlobalVariable.Destination
            , ('StartDate') : GlobalVariable.StartDate1, ('TokenNumber') : GlobalVariable.TokenNumber, ('EndDate') : GlobalVariable.EndDate]))

WS.verifyResponseStatusCode(Response12, 200)

Response13 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteStandard', 
        [('TokenNumber') : GlobalVariable.TokenNumber, ('Provider') : GlobalVariable.Provider, ('PCC') : GlobalVariable.PCC1]))

WS.verifyResponseStatusCode(Response13, 200)

Response14 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteStorePrice', 
        [('Username') : GlobalVariable.Username, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('Carrier') : GlobalVariable.Carrier
            , ('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response14, 200)

Response15 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/PNRBFManagement_PNRRetrieve2', 
        [('Username') : GlobalVariable.Username, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber
            , ('RecordLoc') : GlobalVariable.RecordLoc]))

WS.verifyResponseStatusCode(Response15, 200)

Response16 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/DocProdFareManipulation', 
        [('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response16, 200)

Response17 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/Endsession', 
        [('Username') : GlobalVariable.Username, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response17, 200)

Response18 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/Availability2', 
        [('Profile') : GlobalVariable.Profile1, ('Carrier') : findTestData('Air_1G').getValue(4, 1), ('Origin') : findTestData(
                'Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1), ('EndDate') : GlobalVariable.EndDate]))

WS.verifyResponseStatusCode(Response18, 200)

String xml4 = Response18.responseBodyContent

def result4 = new XmlSlurper().parseText(xml4)

println('result4' + result4)

GlobalVariable.Carrier2 = result4.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].AirV.text()

println('Carrier' + GlobalVariable.Carrier2)

String FlightNumber2 = result4.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].FltNum.text()

if (FlightNumber2.length() < 4) {
    FlightNumber2 = ('0' + FlightNumber2)
}

GlobalVariable.FlightNumber2 = FlightNumber2

GlobalVariable.StartDate2 = result4.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].StartDt.text()

GlobalVariable.Origin2 = result4.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].StartAirp.text()

GlobalVariable.Destination2 = result4.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].EndAirp.text()

String StartTime2 = result4.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].StartTm.text()

if (StartTime2.length() < 4) {
    StartTime2 = ('0' + StartTime2)
}

GlobalVariable.StartTime2 = StartTime2

String EndTime2 = result4.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].EndTm.text()

if (EndTime2.length() < 4) {
    EndTime2 = ('0' + EndTime2)
}

GlobalVariable.EndTime2 = EndTime2

Response19 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteClassSpecific', 
        [('Carrier') : GlobalVariable.Carrier, ('FlightNumber') : GlobalVariable.FlightNumber1, ('Origin') : GlobalVariable.Origin
            , ('Destination') : GlobalVariable.Destination, ('StartDate') : GlobalVariable.StartDate1, ('StartTime') : GlobalVariable.StartTime1
            , ('EndTime') : GlobalVariable.EndTime1, ('Profile') : GlobalVariable.Profile1, ('Carrier2') : GlobalVariable.Carrier2
            , ('FlightNumber2') : GlobalVariable.FlightNumber2, ('Origin2') : GlobalVariable.Origin2, ('Destination2') : GlobalVariable.Destination2
            , ('StartTime2') : GlobalVariable.StartTime2, ('EndTime2') : GlobalVariable.EndTime2, ('EndDate') : GlobalVariable.StartDate2]))

WS.verifyResponseStatusCode(Response19, 200)

Response20 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteClassSpecificInt', 
        [('Carrier') : GlobalVariable.Carrier, ('FlightNumber') : GlobalVariable.FlightNumber1, ('Origin') : GlobalVariable.Origin
            , ('Destination') : GlobalVariable.Destination, ('StartDate') : GlobalVariable.StartDate1, ('StartTime') : GlobalVariable.StartTime1
            , ('EndTime') : GlobalVariable.EndTime1, ('Profile') : GlobalVariable.Profile1, ('Carrier2') : GlobalVariable.Carrier2
            , ('FlightNumber2') : GlobalVariable.FlightNumber2, ('Origin2') : GlobalVariable.Origin2, ('Destination2') : GlobalVariable.Destination2
            , ('StartTime2') : GlobalVariable.StartTime2, ('EndTime2') : GlobalVariable.EndTime2, ('EndDate') : GlobalVariable.StartDate2]))

WS.verifyResponseStatusCode(Response20, 200)

Response21 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteFlightSpecificInt', 
        [('Carrier') : GlobalVariable.Carrier, ('FlightNumber') : GlobalVariable.FlightNumber1, ('Origin') : GlobalVariable.Origin
            , ('Destination') : GlobalVariable.Destination, ('StartDate') : GlobalVariable.StartDate1, ('StartTime') : GlobalVariable.StartTime1
            , ('EndTime') : GlobalVariable.EndTime1, ('Profile') : GlobalVariable.Profile1, ('Carrier2') : GlobalVariable.Carrier2
            , ('FlightNumber2') : GlobalVariable.FlightNumber2, ('Origin2') : GlobalVariable.Origin2, ('Destination2') : GlobalVariable.Destination2
            , ('StartTime2') : GlobalVariable.StartTime2, ('EndTime2') : GlobalVariable.EndTime2, ('EndDate') : GlobalVariable.StartDate2]))

WS.verifyResponseStatusCode(Response21, 200)

Response22 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteFlightSpecific', 
        [('Carrier') : GlobalVariable.Carrier, ('FlightNumber') : GlobalVariable.FlightNumber1, ('Origin') : GlobalVariable.Origin
            , ('Destination') : GlobalVariable.Destination, ('StartDate') : GlobalVariable.StartDate1, ('StartTime') : GlobalVariable.StartTime1
            , ('EndTime') : GlobalVariable.EndTime1, ('Profile') : GlobalVariable.Profile1, ('Carrier2') : GlobalVariable.Carrier2
            , ('FlightNumber2') : GlobalVariable.FlightNumber2, ('Origin2') : GlobalVariable.Origin2, ('Destination2') : GlobalVariable.Destination2
            , ('StartTime2') : GlobalVariable.StartTime2, ('EndTime2') : GlobalVariable.EndTime2, ('EndDate') : GlobalVariable.StartDate2]))

WS.verifyResponseStatusCode(Response22, 200)

Response23 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteMultiDisplay', 
        [('Origin') : GlobalVariable.Origin, ('Destination') : GlobalVariable.Destination, ('StartDate') : GlobalVariable.StartDate1
            , ('Profile') : GlobalVariable.Profile1]))

WS.verifyResponseStatusCode(Response23, 200)

Response24 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteSuperBB', 
        [('Origin') : GlobalVariable.Origin, ('Destination') : GlobalVariable.Destination, ('StartDate') : GlobalVariable.StartDate1
            , ('Origin2') : GlobalVariable.Origin2, ('Destination2') : GlobalVariable.Destination2, ('EndDate') : GlobalVariable.StartDate2
            , ('Profile') : GlobalVariable.Profile1, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider]))

WS.verifyResponseStatusCode(Response24, 200)

Response25 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteSuperBBInt', 
        [('Profile') : GlobalVariable.Profile1, ('Origin') : findTestData('Air_1G').getValue(2, 1), ('Destination') : findTestData(
                'Air_1G').getValue(3, 1), ('Carrier1') : findTestData('Air_1G').getValue(4, 1), ('Carrier2') : findTestData(
                'Air_1G').getValue(5, 1), ('Carrier3') : findTestData('Air_1G').getValue(6, 1), ('StartDate') : GlobalVariable.StartDate
            , ('EndDate') : GlobalVariable.EndDate]))

WS.verifyResponseStatusCode(Response25, 200)

Response26 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/FareQuoteFlexShop', 
        [('Profile') : GlobalVariable.Profile1, ('Origin') : findTestData('Air_1G').getValue(2, 1), ('Destination') : findTestData(
                'Air_1G').getValue(3, 1), ('StartDate') : GlobalVariable.StartDate, ('EndDate') : GlobalVariable.EndDate]))

WS.verifyResponseStatusCode(Response26, 200)

Response27 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/TicketInfoDisplay', 
        [('Profile') : GlobalVariable.Profile1, ('RecordLoc') : GlobalVariable.RecordLoc]))

WS.verifyResponseStatusCode(Response27, 200)

String xml5 = Response27.responseBodyContent

def result5 = new XmlSlurper().parseText(xml5)

println('result5' + result5)

GlobalVariable.TicketNumber = result5.SubmitXmlResult.TicketInfoDisplay_20.ValidTicketInfo.ETicketNum[0].FirstTkNum.text()

GlobalVariable.TicketingCarrier = result5.SubmitXmlResult.TicketInfoDisplay_20.ValidTicketInfo.AirSegInfo[0].AirV.text()

Response28 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/PNRBFManagement_PNRRetrieve3', 
        [('Profile') : GlobalVariable.Profile1, ('RecordLoc') : GlobalVariable.RecordLoc, ('Provider') : GlobalVariable.Provider]))

WS.verifyResponseStatusCode(Response28, 200)

Response29 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLSelect_Air E2E Flow_1G/ETicketRetrieval', 
        [('Profile') : GlobalVariable.Profile1, ('TicketNumber') : GlobalVariable.TicketNumber]))

WS.verifyResponseStatusCode(Response29, 200)

