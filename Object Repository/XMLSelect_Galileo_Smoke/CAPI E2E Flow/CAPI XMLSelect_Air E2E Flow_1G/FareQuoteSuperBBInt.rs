<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>FareQuoteSuperBBInt</name>
   <tag></tag>
   <elementGuidId>48508f09-f4a8-47f6-a00a-870503cd59af</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic R1dTX1VBVF9QNzExMzE2MDpOZXdwYXNzXzR5b3U=</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:web=&quot;http://webservices.galileo.com&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;web:SubmitXml>
         &lt;web:Profile>${Profile}&lt;/web:Profile>
         &lt;web:Request>  
&lt;FareQuoteSuperBBInt_18>
    &lt;AirAvailMods>&lt;!--OD Pair 1(Outbound/Destination Pair)-->
        &lt;AirVPrefInd>
            &lt;AirVIncExcInd>I&lt;/AirVIncExcInd>
            &lt;RelaxAirVPref>Y&lt;/RelaxAirVPref>
        &lt;/AirVPrefInd>
        &lt;AirVPrefs>
            &lt;AirVAry>
                &lt;AirVInfo>
                  &lt;AirV>${Carrier1}&lt;/AirV>
                &lt;/AirVInfo>
                &lt;AirVInfo>
                    &lt;AirV>${Carrier2}&lt;/AirV>
                &lt;/AirVInfo>
                &lt;AirVInfo>
                    &lt;AirV>${Carrier3}&lt;/AirV>
                &lt;/AirVInfo>
            &lt;/AirVAry>
        &lt;/AirVPrefs>
        &lt;GenAvail>
            &lt;NumSeats>2&lt;/NumSeats>
            &lt;Class>Y&lt;/Class>
            &lt;StartDt>${StartDate}&lt;/StartDt>
            &lt;StartPt>${Origin}&lt;/StartPt>
            &lt;EndPt>${Destination}&lt;/EndPt>
            &lt;StartTm>0000&lt;/StartTm>
            &lt;TmWndInd>D&lt;/TmWndInd>
            &lt;StartTmWnd>0000&lt;/StartTmWnd>
            &lt;EndTmWnd>2359&lt;/EndTmWnd>
            &lt;JrnyTm>&lt;/JrnyTm>
            &lt;FltTypeInd>&lt;/FltTypeInd>
            &lt;FltTypePref>&lt;/FltTypePref>
            &lt;StartPtInd>N&lt;/StartPtInd>
            &lt;EndPtInd>N&lt;/EndPtInd>
            &lt;IgnoreTSPref>N&lt;/IgnoreTSPref>
        &lt;/GenAvail>
    &lt;/AirAvailMods>
    &lt;AirAvailMods>&lt;!--OD Pair 2(Outbound/Destination Pair)-->
        &lt;AirVPrefInd>
            &lt;AirVIncExcInd>I&lt;/AirVIncExcInd>
            &lt;RelaxAirVPref>Y&lt;/RelaxAirVPref>
        &lt;/AirVPrefInd>
        &lt;AirVPrefs>
            &lt;AirVAry>
                &lt;AirVInfo>
                  &lt;AirV>${Carrier1}&lt;/AirV>
                &lt;/AirVInfo>
                &lt;AirVInfo>
                    &lt;AirV>${Carrier2}&lt;/AirV>
                &lt;/AirVInfo>
                &lt;AirVInfo>
                    &lt;AirV>${Carrier3}&lt;/AirV>
                &lt;/AirVInfo>
            &lt;/AirVAry>
        &lt;/AirVPrefs>
        &lt;GenAvail>
            &lt;NumSeats>2&lt;/NumSeats>
            &lt;Class>Y&lt;/Class>
            &lt;StartDt>${EndDate}&lt;/StartDt>
            &lt;StartPt>${Destination}&lt;/StartPt>
            &lt;EndPt>${Origin}&lt;/EndPt>
            &lt;StartTm>0000&lt;/StartTm>
            &lt;TmWndInd>D&lt;/TmWndInd>
            &lt;StartTmWnd>0000&lt;/StartTmWnd>
            &lt;EndTmWnd>2359&lt;/EndTmWnd>
            &lt;JrnyTm>&lt;/JrnyTm>
            &lt;FltTypeInd>&lt;/FltTypeInd>
            &lt;FltTypePref>&lt;/FltTypePref>
            &lt;StartPtInd>N&lt;/StartPtInd>
            &lt;EndPtInd>N&lt;/EndPtInd>
            &lt;IgnoreTSPref>N&lt;/IgnoreTSPref>
        &lt;/GenAvail>
    &lt;/AirAvailMods>
    &lt;SuperBBMods>&lt;!--This section checks for fares-->
        &lt;ClassPrefs>&lt;!--Required for Class Specific-->
            &lt;ODPairAry>
                &lt;ODPair>&lt;!--Required for each ODPair(Outbound/Destination Pair) for Class Specific-->
                    &lt;ODNum>1&lt;/ODNum>&lt;!--Incremental number required to reference 1st OD Pair NYCDUB-->
                    &lt;ClassPref>Y&lt;/ClassPref>
                &lt;/ODPair>
                &lt;ODPair>&lt;!--Required for each ODPair(Outbound/Destination Pair) for Class Specific-->
                    &lt;ODNum>2&lt;/ODNum>&lt;!--Incremental number required to reference 2ndOD Pair DUBNYC-->
                    &lt;ClassPref>Y&lt;/ClassPref>
                &lt;/ODPair>
            &lt;/ODPairAry>
        &lt;/ClassPrefs>
        &lt;PassengerType>
            &lt;PsgrAry>
                &lt;Psgr>
                    &lt;LNameNum>1&lt;/LNameNum>
                    &lt;PsgrNum>1&lt;/PsgrNum>
                    &lt;AbsNameNum>1&lt;/AbsNameNum>
                    &lt;PTC>&lt;/PTC>&lt;!--Blank defaults to Adult-->
                    &lt;TIC>&lt;/TIC>
                &lt;/Psgr>
                &lt;Psgr>
                    &lt;LNameNum>1&lt;/LNameNum>
                    &lt;PsgrNum>2&lt;/PsgrNum>
                    &lt;AbsNameNum>2&lt;/AbsNameNum>
                    &lt;PTC>CNN&lt;/PTC>
                    &lt;Age>04&lt;/Age>&lt;!--4 year old Child-->
                    &lt;TIC>&lt;/TIC>
                &lt;/Psgr>
                &lt;Psgr>
                    &lt;LNameNum>1&lt;/LNameNum>
                    &lt;PsgrNum>3&lt;/PsgrNum>
                    &lt;AbsNameNum>3&lt;/AbsNameNum>
                    &lt;PTC>INF&lt;/PTC>
                    &lt;Age>01&lt;/Age>&lt;!-- 1 year old Infant  -->
                    &lt;TIC>&lt;/TIC>
                &lt;/Psgr>
            &lt;/PsgrAry>
        &lt;/PassengerType>
        &lt;Optimize>
            &lt;RecType>1001&lt;/RecType>
            &lt;KlrIDAry>
                &lt;KlrID>AAFI&lt;/KlrID>&lt;!--Returns Available Flight Information-->
                &lt;KlrID>AABC&lt;/KlrID>&lt;!--Returns info on availabilty of First, Business, and Coach Classes, A=available, N=not available-->
                &lt;KlrID>AAB1&lt;/KlrID>&lt;!--Returns list of BIC codes available in each class-->
            &lt;/KlrIDAry>
        &lt;/Optimize>
        &lt;Optimize>
            &lt;RecType>1425&lt;/RecType>
            &lt;KlrIDAry>
                &lt;KlrID>EROR&lt;/KlrID>&lt;!--Error Information-->
                &lt;KlrID>GFGQ&lt;/KlrID>&lt;!--GenQuoteDetails-->
                &lt;KlrID>GFXI&lt;/KlrID>&lt;!--FlightItemCrossReference-->
                &lt;KlrID>GRFB&lt;/KlrID>&lt;!--FareBasisCodeSummary-->
                &lt;KlrID>GROM&lt;/KlrID>&lt;!--Output Text on errors only-->
                &lt;KlrID>GFFC&lt;/KlrID>&lt;!--FareConstruction-->
                &lt;KlrID>GFMM&lt;/KlrID>&lt;!--InfoMsg-->
                &lt;KlrID>GFPI&lt;/KlrID>&lt;!--PsgrTypes-->
                &lt;KlrID>GFRI&lt;/KlrID>&lt;!--RulesInfo-->
                &lt;KlrID>GFRR&lt;/KlrID>&lt;!--RsvnRules-->
                &lt;KlrID>GFSR&lt;/KlrID>&lt;!--SegRelatedInfo-->
                &lt;KlrID>GFSU&lt;/KlrID>&lt;!--Surcharge-->
                &lt;KlrID>GFTS&lt;/KlrID>&lt;!--TaxBreakdown-->
                &lt;KlrID>GFPF&lt;/KlrID>&lt;!--PsgrFacilityCharge-->
            &lt;/KlrIDAry>
        &lt;/Optimize>
    &lt;/SuperBBMods>
&lt;/FareQuoteSuperBBInt_18>


   &lt;/web:Request>
         &lt;web:Filter>
            &lt;_/>
         &lt;/web:Filter>
      &lt;/web:SubmitXml>
      &lt;web:BeginSession/>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${GlobalVariable.Endpoint}</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Profile</defaultValue>
      <description></description>
      <id>a08c6f20-efff-4834-8b43-d49d41c68c6c</id>
      <masked>false</masked>
      <name>Profile</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(2, 1)</defaultValue>
      <description></description>
      <id>0b8783ca-50c8-4eb1-a0d3-f6c5f53ad048</id>
      <masked>false</masked>
      <name>Origin</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(3, 1)</defaultValue>
      <description></description>
      <id>f6f6bd90-d902-4d7e-a44e-2461b8d05afa</id>
      <masked>false</masked>
      <name>Destination</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(4, 1)</defaultValue>
      <description></description>
      <id>e86fc02d-c604-47a4-b69a-537147211455</id>
      <masked>false</masked>
      <name>Carrier1</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(5, 1)</defaultValue>
      <description></description>
      <id>2ec30b0d-39c6-4469-b3ad-e1d853a503a8</id>
      <masked>false</masked>
      <name>Carrier2</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(6, 1)</defaultValue>
      <description></description>
      <id>9db3b881-53f6-44a2-a3eb-27c029fbd8f6</id>
      <masked>false</masked>
      <name>Carrier3</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.StartDate</defaultValue>
      <description></description>
      <id>432995f9-e8ae-45b9-9852-1fada0ee37f4</id>
      <masked>false</masked>
      <name>StartDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.EndDate</defaultValue>
      <description></description>
      <id>5f4d9c63-32ba-45b3-bfad-2ab7d6bb6ea4</id>
      <masked>false</masked>
      <name>EndDate</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

println(&quot;QuoteNumber : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBBInt_18.FareInfo.GenQuoteDetails.QuoteNum')))
println(&quot;QuoteType : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBBInt_18.FareInfo.GenQuoteDetails.QuoteType')))
println(&quot;LastTkDt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBBInt_18.FareInfo.GenQuoteDetails.LastTkDt')))
println(&quot;QuoteDt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBBInt_18.FareInfo.GenQuoteDetails.QuoteDt')))
println(&quot;BaseFareCurrency : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBBInt_18.FareInfo.GenQuoteDetails.BaseFareCurrency')))
println(&quot;BaseFareAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBBInt_18.FareInfo.GenQuoteDetails.BaseFareAmt')))
println(&quot;EquivCurrency : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBBInt_18.FareInfo.GenQuoteDetails.EquivCurrency')))
println(&quot;EquivAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBBInt_18.FareInfo.GenQuoteDetails.EquivAmt')))
println(&quot;TotCurrency : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBBInt_18.FareInfo.GenQuoteDetails.TotCurrency')))
println(&quot;TotAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBBInt_18.FareInfo.GenQuoteDetails.TotAmt')))
println(&quot;TaxInfo : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBBInt_18.FareInfo.GenQuoteDetails.TaxDataAry.TaxData.Country')))
println(&quot;TaxAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBBInt_18.FareInfo.GenQuoteDetails.TaxDataAry.TaxData.Amt')))</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
