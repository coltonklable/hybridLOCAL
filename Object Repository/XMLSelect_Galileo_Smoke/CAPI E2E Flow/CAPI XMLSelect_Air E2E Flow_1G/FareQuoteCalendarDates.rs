<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>FareQuoteCalendarDates</name>
   <tag></tag>
   <elementGuidId>ff465d53-6d03-495b-86e5-b8159c0c1c09</elementGuidId>
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
      &lt;web:SubmitXmlOnSession>
         &lt;web:Token>${TokenNumber}&lt;/web:Token>
         &lt;web:Request>
            &lt;FareQuoteCalendarDates_30>
               &lt;FareDisplayMods>
                  &lt;QueryHeader>
                     &lt;UniqueKey>0000&lt;/UniqueKey>
                     &lt;LangNum>00&lt;/LangNum>
                     &lt;Action>002&lt;/Action>
                     &lt;RetCRTOutput/>
                     &lt;NoMsg/>
                     &lt;NoTrunc/>
                     &lt;IMInd/>
                     &lt;FIPlus/>
                     &lt;PEInd/>
                     &lt;NBInd/>
                     &lt;ActionOnlyInd/>
                     &lt;TranslatePeriod/>
                     &lt;IntFrame1/>
                     &lt;SmartParsed>Y&lt;/SmartParsed>
                     &lt;PDCodes/>
                     &lt;BkDtOverride/>
                     &lt;HostUse25/>
                     &lt;DefCurrency/>
                     &lt;PFPWInd/>
                     &lt;HostUse29/>
                     &lt;HostUse30/>
                     &lt;HostUse31/>
                     &lt;HostUse33/>
                     &lt;TariffQual>
                        &lt;PrefDataRequired>Y&lt;/PrefDataRequired>
                     &lt;/TariffQual>
                  &lt;/QueryHeader>
                  &lt;TravConstraints>
                     &lt;UniqueKey>0000&lt;/UniqueKey>
                     &lt;StartPt>${Origin}&lt;/StartPt>
                     &lt;EndPt>${Destination}&lt;/EndPt>
                     &lt;OW>N&lt;/OW>
                     &lt;RT>Y&lt;/RT>
                     &lt;LongDispInd>N&lt;/LongDispInd>
                     &lt;ValidatingDispInd>N&lt;/ValidatingDispInd>
                     &lt;NUCInd>N&lt;/NUCInd>
                     &lt;RetDataInd>N&lt;/RetDataInd>
                     &lt;BaseFares>N&lt;/BaseFares>
                     &lt;ConxPts>N&lt;/ConxPts>
                     &lt;IncDomTax>N&lt;/IncDomTax>
                     &lt;ConvAP>N&lt;/ConvAP>
                     &lt;FQSFareType>N&lt;/FQSFareType>
                     &lt;HalfRT>N&lt;/HalfRT>
                     &lt;CalShopReq/>
                     &lt;CheckAltCitiesExist>N&lt;/CheckAltCitiesExist>
                     &lt;RetAltCityQuote>N&lt;/RetAltCityQuote>
                     &lt;StartDt>${StartDate}&lt;/StartDt>
                     &lt;AirV1/>
                     &lt;AirV2/>
                     &lt;AirV3/>
                     &lt;GlobDir/>
                     &lt;ConxPt1/>
                     &lt;ConxPt2/>
                     &lt;EndDt>${EndDate}&lt;/EndDt>
                     &lt;TkDt>&lt;/TkDt>
                     &lt;FareType/>
                     &lt;Currency/>
                     &lt;Pt/>
                     &lt;SellCurrency/>
                     &lt;JointFares>N&lt;/JointFares>
                     &lt;RndWorld>N&lt;/RndWorld>
                     &lt;CircTrip>N&lt;/CircTrip>
                     &lt;DoubleOneWay/>
                     &lt;AltDatesReq/>
                     &lt;Surcharges>N&lt;/Surcharges>
                     &lt;Spare1>N&lt;/Spare1>
                     &lt;SkipEffDtProcess/>
                     &lt;CabinNum>4&lt;/CabinNum>
                     &lt;EffStartDtFilter/>
                     &lt;EffEndDtFilter/>
                  &lt;/TravConstraints>
                  &lt;TariffMods>
                     &lt;UniqueKey>0000&lt;/UniqueKey>
                     &lt;StartCity>${Origin}&lt;/StartCity>
                     &lt;EndCity>${Destination}&lt;/EndCity>
                     &lt;Dt>${StartDate}&lt;/Dt>
                     &lt;FIC>SLE4Q&lt;/FIC>
                     &lt;AirV1>${Carrier}&lt;/AirV1>
                     &lt;AirV2/>
                     &lt;ReverseDispRequested>N&lt;/ReverseDispRequested>
                     &lt;AllParagraphsRequested>Y&lt;/AllParagraphsRequested>
                     &lt;SummaryRuleDispRequested>Y&lt;/SummaryRuleDispRequested>
                     &lt;Spare1>N&lt;/Spare1>
                     &lt;HistReq>N&lt;/HistReq>
                     &lt;OutCalendar>Y&lt;/OutCalendar>
                     &lt;InCalendar>N&lt;/InCalendar>
                     &lt;Spare2>N&lt;/Spare2>
                  &lt;/TariffMods>
               &lt;/FareDisplayMods>
            &lt;/FareQuoteCalendarDates_30>
         &lt;/web:Request>
         &lt;web:Filter>
            &lt;_/>
         &lt;/web:Filter>
      &lt;/web:SubmitXmlOnSession>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${GlobalVariable.Endpoint}</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Carrier</defaultValue>
      <description></description>
      <id>8384aa5b-5ca2-4cf5-ab9d-c581492e73cc</id>
      <masked>false</masked>
      <name>Carrier</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Origin</defaultValue>
      <description></description>
      <id>ef98554a-4288-4b00-8f9f-aa05393b3d43</id>
      <masked>false</masked>
      <name>Origin</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Destination</defaultValue>
      <description></description>
      <id>c2ff2772-6402-47d5-82bb-16fc074160d4</id>
      <masked>false</masked>
      <name>Destination</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.StartDate1</defaultValue>
      <description></description>
      <id>c2f440ab-c332-452a-8b3b-8383946a3d38</id>
      <masked>false</masked>
      <name>StartDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TokenNumber</defaultValue>
      <description></description>
      <id>a08c6f20-efff-4834-8b43-d49d41c68c6c</id>
      <masked>false</masked>
      <name>TokenNumber</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.EndDate</defaultValue>
      <description></description>
      <id>85e4602c-1623-446b-906a-8e571a1949ad</id>
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

println(&quot;Text : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteCalendarDates_30.FareInfo.InfoMsg.Text')))
println(&quot;Fare : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteCalendarDates_30.FareInfo.Tariff.Type1Qual.Fare')))
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
