<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>FareQuoteClassSpecificInt</name>
   <tag></tag>
   <elementGuidId>11f56065-c182-4a0f-88f7-6b0cb1d8f9fa</elementGuidId>
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
            
&lt;FareQuoteClassSpecificInt_15>
    &lt;ClassSpecificMods>
        &lt;SegInfo>
            &lt;FltSegAry>
                   &lt;FltSeg>
                           &lt;ClassPref>Y&lt;/ClassPref>
                           &lt;AirV>${Carrier}&lt;/AirV>
                           &lt;FltNum>${FlightNumber}&lt;/FltNum>
                           &lt;OpSuf/>
                           &lt;Dt>${StartDate}&lt;/Dt>
                           &lt;StartAirp>${Origin}&lt;/StartAirp>
                           &lt;EndAirp>${Destination}&lt;/EndAirp>
                           &lt;StartTm>${StartTime}&lt;/StartTm>
                           &lt;EndTm>${EndTime}&lt;/EndTm>
                           &lt;AsBookedBIC>Q&lt;/AsBookedBIC>
                           &lt;DayChgInd>00&lt;/DayChgInd>
                           &lt;Conx>Y&lt;/Conx>
                        &lt;/FltSeg>
                       &lt;FltSeg>
                           &lt;ClassPref>Y&lt;/ClassPref>
                           &lt;AirV>${Carrier2}&lt;/AirV>
                           &lt;FltNum>${FlightNumber2}&lt;/FltNum>
                           &lt;OpSuf/>
                           &lt;Dt>${EndDate}&lt;/Dt>
                           &lt;StartAirp>${Origin2}&lt;/StartAirp>
                           &lt;EndAirp>${Destination2}&lt;/EndAirp>
                           &lt;StartTm>${StartTime2}&lt;/StartTm>
                           &lt;EndTm>${EndTime2}&lt;/EndTm>
                           &lt;AsBookedBIC>Q&lt;/AsBookedBIC>
                           &lt;DayChgInd>00&lt;/DayChgInd>
                           &lt;Conx>Y&lt;/Conx>
                        &lt;/FltSeg>
             &lt;/FltSegAry>
         &lt;/SegInfo>
&lt;BrandModifier>
&lt;Brand>
&lt;BrandElement>
&lt;StartSeg>01&lt;/StartSeg>
&lt;EndSeg>01&lt;/EndSeg>
&lt;PricebyBrandModifier>0002&lt;/PricebyBrandModifier>
&lt;/BrandElement>
&lt;/Brand>
&lt;/BrandModifier>
         &lt;PassengerType>
             &lt;PsgrAry>
                 &lt;Psgr>
                    &lt;LNameNum>1&lt;/LNameNum>
                    &lt;PsgrNum>1&lt;/PsgrNum>
                    &lt;AbsNameNum>1&lt;/AbsNameNum>
                    &lt;PTC>ADT&lt;/PTC>
                    &lt;TIC/>
                 &lt;/Psgr>
             &lt;/PsgrAry>
         &lt;/PassengerType>
         &lt;GenQuoteInfo>
			 &lt;RulesProcess>Y&lt;/RulesProcess>
         &lt;/GenQuoteInfo>
     &lt;/ClassSpecificMods>
&lt;/FareQuoteClassSpecificInt_15>


         &lt;/web:Request>
         &lt;web:Filter>
            &lt;_/>
         &lt;/web:Filter>
      &lt;/web:SubmitXml>
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
      <defaultValue>GlobalVariable.FlightNumber1</defaultValue>
      <description></description>
      <id>306cdd05-be21-4991-a55a-743ec00e3ed5</id>
      <masked>false</masked>
      <name>FlightNumber</name>
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
      <defaultValue>GlobalVariable.StartTime1</defaultValue>
      <description></description>
      <id>6adc41bd-a27f-496c-923a-73c7daae6a1d</id>
      <masked>false</masked>
      <name>StartTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.EndTime1</defaultValue>
      <description></description>
      <id>3e65ce01-192c-48b7-a459-a258739e191f</id>
      <masked>false</masked>
      <name>EndTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Profile</defaultValue>
      <description></description>
      <id>a08c6f20-efff-4834-8b43-d49d41c68c6c</id>
      <masked>false</masked>
      <name>Profile</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Carrier2</defaultValue>
      <description></description>
      <id>5aa2a6ff-085f-4a00-aff4-34c76ac9561e</id>
      <masked>false</masked>
      <name>Carrier2</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.FlightNumber2</defaultValue>
      <description></description>
      <id>730472b6-61e6-4f74-bebe-ebc47c66a07b</id>
      <masked>false</masked>
      <name>FlightNumber2</name>
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
      <defaultValue>GlobalVariable.StartTime2</defaultValue>
      <description></description>
      <id>b247a202-c582-428c-9bfa-b76359bf007b</id>
      <masked>false</masked>
      <name>StartTime2</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.EndTime2</defaultValue>
      <description></description>
      <id>9e458a20-5d04-4031-a8e8-9327e29ad143</id>
      <masked>false</masked>
      <name>EndTime2</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.StartDate2</defaultValue>
      <description></description>
      <id>e5ff6bc9-71e4-47d2-97b8-6591d081531b</id>
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

println(&quot;QuoteNumber : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GenQuoteDetails.QuoteNum')))
println(&quot;QuoteType : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GenQuoteDetails.QuoteType')))
println(&quot;LastTkDt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GenQuoteDetails.LastTkDt')))
println(&quot;QuoteDt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GenQuoteDetails.QuoteDt')))
println(&quot;BaseFareCurrency : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GenQuoteDetails.BaseFareCurrency')))
println(&quot;BaseFareAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GenQuoteDetails.BaseFareAmt')))
println(&quot;EquivCurrency : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GenQuoteDetails.EquivCurrency')))
println(&quot;EquivAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GenQuoteDetails.EquivAmt')))
println(&quot;TotCurrency : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GenQuoteDetails.TotCurrency')))
println(&quot;TotAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GenQuoteDetails.TotAmt')))
println(&quot;TaxInfo : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GenQuoteDetails.TaxDataAry.TaxData.Country')))
println(&quot;TaxAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GenQuoteDetails.TaxDataAry.TaxData.Amt')))
println(&quot;Currency : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GrandFeeTotal.Crncy')))
println(&quot;SubTotAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GrandFeeTotal.SubTotAmt')))
println(&quot;GrandTotAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GrandFeeTotal.GrandTotAmt')))
println(&quot;BestBuyCompTotAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GrandFeeTotal.BestBuyCompTotAmt')))
println(&quot;PlatingAirV : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteClassSpecificInt_15.FareInfo.GrandFeeTotal.PlatingAirV')))</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
