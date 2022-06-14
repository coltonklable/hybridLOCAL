<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>FareQuoteDollarSaver</name>
   <tag></tag>
   <elementGuidId>f272bbd8-ee20-4bce-b714-86a5bab55a94</elementGuidId>
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

&lt;FareQuoteDollarSaver_35>
    &lt;DollarSaverMods>
        &lt;PassengerType>
            &lt;PsgrAry>
                &lt;Psgr>
                    &lt;LNameNum>01&lt;/LNameNum>
                    &lt;PsgrNum>01&lt;/PsgrNum>
                    &lt;AbsNameNum>01&lt;/AbsNameNum>
                 &lt;/Psgr>
              &lt;/PsgrAry>
        &lt;/PassengerType>
         &lt;GenQuoteInfo>
             &lt;RulesProcess>Y&lt;/RulesProcess>
        &lt;/GenQuoteInfo>
        &lt;SegSelection>
            &lt;ReqAirVPFs>Y&lt;/ReqAirVPFs>
            &lt;SegRangeAry>
                &lt;SegRange>
                    &lt;StartSeg>01&lt;/StartSeg>
                    &lt;EndSeg>01&lt;/EndSeg>
                    &lt;FareType>N&lt;/FareType>
               &lt;/SegRange>
            &lt;/SegRangeAry>
        &lt;/SegSelection>
    &lt;/DollarSaverMods>
&lt;/FareQuoteDollarSaver_35>
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
      <defaultValue>GlobalVariable.TokenNumber</defaultValue>
      <description></description>
      <id>a08c6f20-efff-4834-8b43-d49d41c68c6c</id>
      <masked>false</masked>
      <name>TokenNumber</name>
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

println(&quot;QuoteNumber : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GenQuoteDetails.QuoteNum')))
println(&quot;QuoteType : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GenQuoteDetails.QuoteType')))
println(&quot;LastTkDt : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GenQuoteDetails.LastTkDt')))
println(&quot;QuoteDt : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GenQuoteDetails.QuoteDt')))
println(&quot;BaseFareCurrency : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GenQuoteDetails.BaseFareCurrency')))
println(&quot;BaseFareAmt : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GenQuoteDetails.BaseFareAmt')))
println(&quot;EquivCurrency : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GenQuoteDetails.EquivCurrency')))
println(&quot;EquivAmt : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GenQuoteDetails.EquivAmt')))
println(&quot;TotCurrency : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GenQuoteDetails.TotCurrency')))
println(&quot;TotAmt : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GenQuoteDetails.TotAmt')))
println(&quot;TaxInfo : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GenQuoteDetails.TaxDataAry.TaxData.Country')))
println(&quot;TaxAmt : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GenQuoteDetails.TaxDataAry.TaxData.Amt')))
println(&quot;Currency : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GrandFeeTotal.Crncy')))
println(&quot;SubTotAmt : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GrandFeeTotal.SubTotAmt')))
println(&quot;GrandTotAmt : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GrandFeeTotal.GrandTotAmt')))
println(&quot;BestBuyCompTotAmt : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GrandFeeTotal.BestBuyCompTotAmt')))
println(&quot;PlatingAirV : &quot; + (WS.getElementText(response, 'SubmitXmlOnSessionResponse.SubmitXmlOnSessionResult.FareQuoteDollarSaver_35.FareInfo.GrandFeeTotal.PlatingAirV')))</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
