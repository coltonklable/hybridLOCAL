<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AirAvailabilityByTariff</name>
   <tag></tag>
   <elementGuidId>10e8993d-1030-4d78-a205-d2c276404d19</elementGuidId>
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
          
&lt;AirAvailabilityByTariff_34>
    &lt;FareDisplayMods>
        &lt;QueryHeader>
            &lt;UniqueKey>0000&lt;/UniqueKey>
            &lt;LangNum>00&lt;/LangNum>
            &lt;Action>034&lt;/Action>
        &lt;/QueryHeader>
        &lt;TariffMods>
            &lt;UniqueKey>0000&lt;/UniqueKey>
            &lt;StartCity>${Origin}&lt;/StartCity>
            &lt;EndCity>${Destination}&lt;/EndCity>
            &lt;Dt>${StartDate}&lt;/Dt>
            &lt;FIC>GRA2PKER&lt;/FIC>
            &lt;AirV1>${Carrier}&lt;/AirV1>
            &lt;AirV2>&lt;/AirV2>
            &lt;ReverseDispRequested>N&lt;/ReverseDispRequested>
            &lt;AllParagraphsRequested>N&lt;/AllParagraphsRequested>
            &lt;SummaryRuleDispRequested>Y&lt;/SummaryRuleDispRequested>
            &lt;Spare1>N&lt;/Spare1>
            &lt;HistReq>N&lt;/HistReq>
            &lt;OutCalendar>Y&lt;/OutCalendar>
            &lt;InCalendar>N&lt;/InCalendar>
            &lt;Spare2>N&lt;/Spare2>
        &lt;/TariffMods>
    &lt;/FareDisplayMods>
    &lt;AirAvailMods>
        &lt;GenAvail>
            &lt;NumSeats>1&lt;/NumSeats>
            &lt;StartDt>${StartDate}&lt;/StartDt>
            &lt;StartPt>${Origin}&lt;/StartPt>
            &lt;EndPt>${Destination}&lt;/EndPt>
            &lt;StartTm>0001&lt;/StartTm>
            &lt;TmWndInd>D&lt;/TmWndInd>
            &lt;StartTmWnd>000&lt;/StartTmWnd>
            &lt;EndTmWnd>2359&lt;/EndTmWnd>
            &lt;JrnyTm/>
            &lt;IgnoreTSPref>Y&lt;/IgnoreTSPref>
            &lt;MaxNumFlts>16&lt;/MaxNumFlts>
        &lt;/GenAvail>
        &lt;BICFilter>
            &lt;BICFilterAry>
                &lt;BICFilterInfo>
                    &lt;BIC>Q&lt;/BIC>
                    &lt;AllowSmlrInd>N&lt;/AllowSmlrInd>
                &lt;/BICFilterInfo>
            &lt;/BICFilterAry>
        &lt;/BICFilter>
        &lt;AirVPrefInd>
            &lt;AirVIncExcInd>O&lt;/AirVIncExcInd>
            &lt;RelaxAirVPref>N&lt;/RelaxAirVPref>
            &lt;SectorNum>0&lt;/SectorNum>
        &lt;/AirVPrefInd>
        &lt;AirVPrefs>
            &lt;AirVAry>
                &lt;AirVInfo>
                    &lt;AirV>${Carrier}&lt;/AirV>
                    &lt;FltRangeStart>0000&lt;/FltRangeStart>
                    &lt;FltRangeEnd>9999&lt;/FltRangeEnd>
                &lt;/AirVInfo>
            &lt;/AirVAry>
        &lt;/AirVPrefs>
    &lt;/AirAvailMods>
&lt;/AirAvailabilityByTariff_34>
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
      <id>3ef50284-4531-4ff5-a29f-003b398060e1</id>
      <masked>false</masked>
      <name>Profile</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(4, 1)</defaultValue>
      <description></description>
      <id>2a26971f-d8de-4459-8201-ee3557887cba</id>
      <masked>false</masked>
      <name>Carrier</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(2, 1)</defaultValue>
      <description></description>
      <id>6260ebbd-22ad-401f-a75f-6705ecf74b44</id>
      <masked>false</masked>
      <name>Origin</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(3, 1)</defaultValue>
      <description></description>
      <id>c6974240-9913-4111-8db0-6c9ff9d8bc31</id>
      <masked>false</masked>
      <name>Destination</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Username</defaultValue>
      <description></description>
      <id>0a023d0a-232d-4ed0-876f-33d2edd7a912</id>
      <masked>false</masked>
      <name>Username</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.StartDate</defaultValue>
      <description></description>
      <id>bffe59d5-46a8-44bc-b53b-768f1313011d</id>
      <masked>false</masked>
      <name>StartDate</name>
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

println(&quot;AirV Values are &quot; + (WS.getElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.AirAvailabilityByTariff_34.AirAvail.AvailFlt.AirV')))
println(&quot;FlightNumber &quot; + (WS.getElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.AirAvailabilityByTariff_34.AirAvail.AvailFlt[0].FltNum')))
println(&quot;StartDate &quot; + (WS.getElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.AirAvailabilityByTariff_34.AirAvail.AvailFlt[0].StartDt')))
println(&quot;Origin &quot; + (WS.getElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.AirAvailabilityByTariff_34.AirAvail.AvailFlt[0].StartAirp')))
println(&quot;Destination &quot; + (WS.getElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.AirAvailabilityByTariff_34.AirAvail.AvailFlt[0].EndAirp')))
println(&quot;StartTime &quot; + (WS.getElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.AirAvailabilityByTariff_34.AirAvail.AvailFlt[0].StartTm')))
println(&quot;EndTime &quot; + (WS.getElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.AirAvailabilityByTariff_34.AirAvail.AvailFlt[0].EndTm')))
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
