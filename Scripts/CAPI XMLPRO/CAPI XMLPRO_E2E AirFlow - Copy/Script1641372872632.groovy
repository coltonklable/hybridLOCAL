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

GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyy-MM-dd', GlobalVariable.DateValue1)

println('StartDate' + GlobalVariable.StartDate)

PSResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/PSC5', 
        [('StartDate') : GlobalVariable.StartDate, ('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Origin') : findTestData(
                'Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1), ('Password') : GlobalVariable.Password
            , ('Username') : GlobalVariable.Username]))

AvailResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/OTA_AirAvailRQ', 
        [('StartDate') : GlobalVariable.StartDate, ('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Origin') : findTestData(
                'Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1), ('Password') : GlobalVariable.Password
            , ('Username') : GlobalVariable.Username]))

String xml1 = AvailResponse.responseText

println('xml1' + xml1)

def result1 = new XmlSlurper().parseText(xml1)

println('result1' + result1)

GlobalVariable.FlightNumber1 = result1.Body.ProviderTransactionResponse.RSP.OTA_AirAvailRS.OriginDestinationInformation.OriginDestinationOptions.OriginDestinationOption.FlightSegment[0].@FlightNumber

println('FlightNumber1' + GlobalVariable.FlightNumber1)

GlobalVariable.DepartureDateTime = result1.Body.ProviderTransactionResponse.RSP.OTA_AirAvailRS.OriginDestinationInformation.OriginDestinationOptions.OriginDestinationOption.FlightSegment[0].@DepartureDateTime

println('DepartureDateTime' + GlobalVariable.DepartureDateTime)

GlobalVariable.ArrivalDateTime = result1.Body.ProviderTransactionResponse.RSP.OTA_AirAvailRS.OriginDestinationInformation.OriginDestinationOptions.OriginDestinationOption.FlightSegment[0].@ArrivalDateTime

println('ArrivalDateTime' + GlobalVariable.ArrivalDateTime)

GlobalVariable.Carrier = result1.Body.ProviderTransactionResponse.RSP.OTA_AirAvailRS.OriginDestinationInformation.OriginDestinationOptions.OriginDestinationOption.FlightSegment[0].MarketingAirline.@Code

println('Carrier' + GlobalVariable.Carrier)

PriceResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/OTA_AirPriceRQ', 
        [('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Password') : GlobalVariable.Password, ('Username') : GlobalVariable.Username
            , ('Origin') : findTestData('Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1)
            , ('DepartureDateTime') : GlobalVariable.DepartureDateTime, ('Carrier') : GlobalVariable.Carrier, ('FlightNumber') : GlobalVariable.FlightNumber1]))

String xml2 = PriceResponse.responseText

println('xml2' + xml2)

def result2 = new XmlSlurper().parseText(xml2)

println('result2' + result2)

GlobalVariable.FareBasisCode = result2.Body.ProviderTransactionResponse.RSP.OTA_AirPriceRS.PricingOverview.PTC_FareBreakdowns.PTC_FareBreakdown.FareInfo.FareInfo[0].@FareBasisCode

println('FareBasisCode' + GlobalVariable.FareBasisCode)

BookResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/OTA_AirBookRQ', 
        [('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Password') : GlobalVariable.Password, ('Username') : GlobalVariable.Username
            , ('Origin') : findTestData('Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1)
            , ('Carrier') : GlobalVariable.Carrier, ('FlightNumber') : GlobalVariable.FlightNumber1, ('DepartureDateTime') : GlobalVariable.DepartureDateTime
            , ('ArrivalDateTime') : GlobalVariable.ArrivalDateTime]))

String xml3 = BookResponse.responseText

println('xml3' + xml3)

def result3 = new XmlSlurper().parseText(xml3)

println('result3' + result3)

GlobalVariable.RecordLoc = result3.Body.ProviderTransactionResponse.RSP.OTA_AirBookRS.AirReservation.BookingReferenceID[0].@ID

println('RecordLoc' + GlobalVariable.RecordLoc)

ScheduleResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/OTA_AirScheduleRQ', 
        [('StartDate') : GlobalVariable.StartDate, ('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Origin') : findTestData(
                'Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1), ('Password') : GlobalVariable.Password
            , ('Username') : GlobalVariable.Username]))

SeatMapResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/OTA_AirSeatMapRQ', 
        [('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Password') : GlobalVariable.Password, ('Username') : GlobalVariable.Username
            , ('Origin') : findTestData('Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1)
            , ('Carrier') : GlobalVariable.Carrier, ('FlightNumber') : GlobalVariable.FlightNumber1, ('DepartureDateTime') : GlobalVariable.DepartureDateTime
            , ('CabinType') : GlobalVariable.CabinType]))

DetailsResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/OTA_AirDetailsRQ', 
        [('StartDate') : GlobalVariable.StartDate, ('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Password') : GlobalVariable.Password
            , ('Username') : GlobalVariable.Username, ('Carrier') : GlobalVariable.Carrier, ('FlightNumber') : GlobalVariable.FlightNumber1]))

FlightInfoResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/OTA_AirFlifoRQ', 
        [('StartDate') : GlobalVariable.StartDate, ('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Password') : GlobalVariable.Password
            , ('Username') : GlobalVariable.Username, ('Carrier') : GlobalVariable.Carrier, ('FlightNumber') : GlobalVariable.FlightNumber1]))

FareDisplayResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/OTA_AirFareDisplayRQ', 
        [('StartDate') : GlobalVariable.StartDate, ('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Origin') : findTestData(
                'Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1), ('Password') : GlobalVariable.Password
            , ('Username') : GlobalVariable.Username]))

RulesResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/OTA_AirRulesRQ', 
        [('StartDate') : GlobalVariable.StartDate, ('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Origin') : findTestData(
                'Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1), ('Password') : GlobalVariable.Password
            , ('Username') : GlobalVariable.Username, ('FareBasisCode') : GlobalVariable.FareBasisCode, ('Carrier') : GlobalVariable.Carrier]))

DPC8Response = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/DPC8', 
        [('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Password') : GlobalVariable.Password, ('Username') : GlobalVariable.Username
            , ('RecordLoc') : GlobalVariable.RecordLoc]))

DemandTicketResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/OTA_AirDemandTicketRQ', 
        [('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Password') : GlobalVariable.Password, ('Username') : GlobalVariable.Username]))

AuthorizationResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/OTA_AuthorizationRQ', 
        [('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Password') : GlobalVariable.Password, ('Username') : GlobalVariable.Username]))

TKC1Response = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/TKC1', 
        [('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Password') : GlobalVariable.Password, ('Username') : GlobalVariable.Username
            , ('RecordLoc') : GlobalVariable.RecordLoc]))

CancelResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/OTA_CancelRQ', 
        [('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Password') : GlobalVariable.Password, ('Username') : GlobalVariable.Username]))

ERC1Response = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/ERC1', 
        [('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Password') : GlobalVariable.Password, ('Username') : GlobalVariable.Username]))

PSC5AResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/PSC5A', 
        [('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Password') : GlobalVariable.Password, ('Username') : GlobalVariable.Username]))

GBC1Response = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/GBC1', 
        [('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Password') : GlobalVariable.Password, ('Username') : GlobalVariable.Username
            , ('Origin') : findTestData('Air_1G').getValue(2, 1), ('Destination') : findTestData('Air_1G').getValue(3, 1)
            , ('Carrier') : GlobalVariable.Carrier]))

ScreenTextResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/OTA_ScreenTextRQ', 
        [('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Password') : GlobalVariable.Password, ('Username') : GlobalVariable.Username]))

CCResponse = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/CAPI E2E Flow/CAPI XMLPRO_Air E2E Flow_1G/CurrencyConversionRQ', 
        [('Session') : GlobalVariable.Session, ('PCC') : GlobalVariable.PCC1, ('Password') : GlobalVariable.Password, ('Username') : GlobalVariable.Username]))

