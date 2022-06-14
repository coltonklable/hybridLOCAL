<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>FareQuoteSuperBB</name>
   <tag></tag>
   <elementGuidId>2beb24bd-5fcc-4c8d-b84d-b64d5f6a458e</elementGuidId>
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
&lt;FareQuoteSuperBB_42>
    &lt;AirAvailMods>
        &lt;GenAvail>
            &lt;NumSeats>2&lt;/NumSeats>
            &lt;Class>&lt;/Class>
            &lt;StartDt>${StartDate}&lt;/StartDt>
            &lt;StartPt>${Origin}&lt;/StartPt>
            &lt;EndPt>${Destination}&lt;/EndPt>
            &lt;StartTm>700&lt;/StartTm>
            &lt;TmWndInd>D&lt;/TmWndInd>
            &lt;StartTmWnd>700&lt;/StartTmWnd>
            &lt;EndTmWnd>1219&lt;/EndTmWnd>
            &lt;JrnyTm/>
            &lt;FltTypeInd>E&lt;/FltTypeInd>
            &lt;FltTypePref>&lt;/FltTypePref>
            &lt;StartPtInd>N&lt;/StartPtInd>
            &lt;EndPtInd>N&lt;/EndPtInd>
            &lt;IgnoreTSPref>N&lt;/IgnoreTSPref>
        &lt;/GenAvail>
    &lt;/AirAvailMods>
    &lt;AirAvailMods>
        &lt;GenAvail>
            &lt;NumSeats>2&lt;/NumSeats>
            &lt;Class>&lt;/Class>
            &lt;StartDt>${EndDate}&lt;/StartDt>
            &lt;StartPt>${Origin2}&lt;/StartPt>
            &lt;EndPt>${Destination2}&lt;/EndPt>
            &lt;StartTm>1305&lt;/StartTm>
            &lt;TmWndInd>D&lt;/TmWndInd>
            &lt;StartTmWnd>1305&lt;/StartTmWnd>
            &lt;EndTmWnd>1428&lt;/EndTmWnd>
             &lt;JrnyTm/>
            &lt;FltTypeInd>E&lt;/FltTypeInd>
            &lt;FltTypePref>&lt;/FltTypePref>
            &lt;StartPtInd>N&lt;/StartPtInd>
            &lt;EndPtInd>N&lt;/EndPtInd>
            &lt;IgnoreTSPref>N&lt;/IgnoreTSPref>
        &lt;/GenAvail>
    &lt;/AirAvailMods>
    &lt;SuperBBMods>
        &lt;PFInfo>
            &lt;ReqAirVPFs>Y&lt;/ReqAirVPFs>
            &lt;PFAry>
                &lt;PF>
                    &lt;StartODRange>00&lt;/StartODRange>
                    &lt;EndODRange>00&lt;/EndODRange>
                    &lt;CRS>${Provider}&lt;/CRS>
                    &lt;PCC>${PCC}&lt;/PCC>
                    &lt;AirV>&lt;/AirV>
                    &lt;Acct>&lt;/Acct>
                    &lt;Contract>&lt;/Contract>
                    &lt;PublishedFaresInd>Y&lt;/PublishedFaresInd>
                    &lt;Type>A&lt;/Type>
                &lt;/PF>
            &lt;/PFAry>
        &lt;/PFInfo>
        &lt;PassengerType>
            &lt;PsgrAry>
                &lt;Psgr>
                    &lt;LNameNum>01&lt;/LNameNum>
                    &lt;PsgrNum>01&lt;/PsgrNum>
                    &lt;AbsNameNum>01&lt;/AbsNameNum>
                    &lt;PTC>&lt;/PTC>
                    &lt;TIC>&lt;/TIC>
                &lt;/Psgr>
            &lt;/PsgrAry>
        &lt;/PassengerType>
        &lt;Optimize>
            &lt;RecType>1425&lt;/RecType>
            &lt;KlrIDAry>
                &lt;KlrID>EROR&lt;/KlrID>
                &lt;KlrID>GFGQ&lt;/KlrID>
                &lt;KlrID>GFXI&lt;/KlrID>
                &lt;KlrID>GRFB&lt;/KlrID>
                &lt;KlrID>GROM&lt;/KlrID>
            &lt;/KlrIDAry>
        &lt;/Optimize>
        &lt;Optimize>
            &lt;RecType>1001&lt;/RecType>
            &lt;KlrIDAry>
                &lt;KlrID>AAFI&lt;/KlrID>
                &lt;KlrID>AATC&lt;/KlrID>
                &lt;KlrID>AATS&lt;/KlrID>
            &lt;/KlrIDAry>
        &lt;/Optimize>
        &lt;Optimize>
            &lt;RecType>1425&lt;/RecType>
            &lt;KlrIDAry>
                &lt;KlrID>GFFC&lt;/KlrID>
                &lt;KlrID>GFMM&lt;/KlrID>
                &lt;KlrID>GFPI&lt;/KlrID>
                &lt;KlrID>GFRH&lt;/KlrID>
                &lt;KlrID>GFRI&lt;/KlrID>
                &lt;KlrID>GFRR&lt;/KlrID>
                &lt;KlrID>GFSR&lt;/KlrID>
                &lt;KlrID>GFSU&lt;/KlrID>
                &lt;KlrID>GFTS&lt;/KlrID>
            &lt;/KlrIDAry>
        &lt;/Optimize>
    &lt;/SuperBBMods>
&lt;/FareQuoteSuperBB_42>
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
      <defaultValue>GlobalVariable.Origin2</defaultValue>
      <description></description>
      <id>e83080e6-72f9-4bea-9f38-4fbbab3ca0d2</id>
      <masked>false</masked>
      <name>Origin2</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Destination2</defaultValue>
      <description></description>
      <id>901fd120-36d6-4d2e-845d-79fd04073c23</id>
      <masked>false</masked>
      <name>Destination2</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.StartDate2</defaultValue>
      <description></description>
      <id>e5ff6bc9-71e4-47d2-97b8-6591d081531b</id>
      <masked>false</masked>
      <name>EndDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Profile</defaultValue>
      <description></description>
      <id>a08c6f20-efff-4834-8b43-d49d41c68c6c</id>
      <masked>false</masked>
      <name>Profile</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PCC</defaultValue>
      <description></description>
      <id>cc0119f9-1956-4b29-a71b-d36dcd129136</id>
      <masked>false</masked>
      <name>PCC</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Provider</defaultValue>
      <description></description>
      <id>38423745-6932-4f00-8806-78bb4b94a4ec</id>
      <masked>false</masked>
      <name>Provider</name>
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

println(&quot;QuoteNumber : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBB_42.FareInfo.GenQuoteDetails.QuoteNum')))
println(&quot;QuoteType : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBB_42.FareInfo.GenQuoteDetails.QuoteType')))
println(&quot;LastTkDt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBB_42.FareInfo.GenQuoteDetails.LastTkDt')))
println(&quot;QuoteDt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBB_42.FareInfo.GenQuoteDetails.QuoteDt')))
println(&quot;BaseFareCurrency : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBB_42.FareInfo.GenQuoteDetails.BaseFareCurrency')))
println(&quot;BaseFareAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBB_42.FareInfo.GenQuoteDetails.BaseFareAmt')))
println(&quot;EquivCurrency : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBB_42.FareInfo.GenQuoteDetails.EquivCurrency')))
println(&quot;EquivAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBB_42.FareInfo.GenQuoteDetails.EquivAmt')))
println(&quot;TotCurrency : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBB_42.FareInfo.GenQuoteDetails.TotCurrency')))
println(&quot;TotAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBB_42.FareInfo.GenQuoteDetails.TotAmt')))
println(&quot;TaxInfo : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBB_42.FareInfo.GenQuoteDetails.TaxDataAry.TaxData.Country')))
println(&quot;TaxAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteSuperBB_42.FareInfo.GenQuoteDetails.TaxDataAry.TaxData.Amt')))</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
