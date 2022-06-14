<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>FareQuoteFlexShop</name>
   <tag></tag>
   <elementGuidId>2f907e22-e6a1-4142-897f-edafcc476811</elementGuidId>
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
      &lt;SubmitXml>
         &lt;Profile>${Profile}&lt;/Profile>
         &lt;Request>
            &lt;FareQuoteFlexShop_21>
               &lt;AirAvailMods>
                  &lt;!--OD Pair 1(Outbound/Destination Pair)-->
                  &lt;!--RT - Flex - Airports - (out) SAN,SFO,SNA - (rtn) FLL,TPA,ATL-->
                  &lt;AirVPrefInd>
                     &lt;AirVIncExcInd>I&lt;/AirVIncExcInd>
                     &lt;RelaxAirVPref>N&lt;/RelaxAirVPref>
                  &lt;/AirVPrefInd>
                  &lt;AirVPrefs>
                     &lt;AirVAry>
                        &lt;AirVInfo>
                           &lt;AirV/>
                        &lt;/AirVInfo>
                        &lt;AirVInfo>
                           &lt;AirV/>
                        &lt;/AirVInfo>
                        &lt;AirVInfo>
                           &lt;AirV/>
                        &lt;/AirVInfo>
                     &lt;/AirVAry>
                  &lt;/AirVPrefs>
                  &lt;GenAvail>
                     &lt;NumSeats>2&lt;/NumSeats>
                     &lt;Class>Y&lt;/Class>
                     &lt;StartDt>${StartDate}&lt;/StartDt>
                     &lt;StartPt>${Origin}&lt;/StartPt>
                     
                     &lt;EndPt>${Destination}&lt;/EndPt>
                     &lt;StartTm>0900&lt;/StartTm>
                     &lt;TmWndInd>D&lt;/TmWndInd>
                     &lt;StartTmWnd>0000&lt;/StartTmWnd>
                     &lt;EndTmWnd>2359&lt;/EndTmWnd>
                     &lt;JrnyTm/>
                     &lt;FltTypeInd/>
                     &lt;FltTypePref/>
                     &lt;StartPtInd>N&lt;/StartPtInd>
                     &lt;EndPtInd>N&lt;/EndPtInd>
                     &lt;IgnoreTSPref>N&lt;/IgnoreTSPref>
                  &lt;/GenAvail>
               &lt;/AirAvailMods>
               &lt;AirAvailMods>
                  &lt;!--OD Pair 2(Outbound/Destination Pair)-->
                  &lt;AirVPrefInd>
                     &lt;AirVIncExcInd>I&lt;/AirVIncExcInd>
                     &lt;RelaxAirVPref>N&lt;/RelaxAirVPref>
                  &lt;/AirVPrefInd>
                  &lt;AirVPrefs>
                     &lt;AirVAry>
                        &lt;AirVInfo>
                           &lt;AirV/>
                        &lt;/AirVInfo>
                        &lt;AirVInfo>
                           &lt;AirV/>
                        &lt;/AirVInfo>
                        &lt;AirVInfo>
                           &lt;AirV/>
                        &lt;/AirVInfo>
                     &lt;/AirVAry>
                  &lt;/AirVPrefs>
                  &lt;GenAvail>
                     &lt;NumSeats>2&lt;/NumSeats>
                     &lt;Class>Y&lt;/Class>
                     &lt;StartDt>${EndDate}&lt;/StartDt>
                     &lt;StartPt>${Destination}&lt;/StartPt>
                     &lt;EndPt>${Origin}&lt;/EndPt>
                     &lt;StartTm>0900&lt;/StartTm>
                     &lt;TmWndInd>D&lt;/TmWndInd>
                     &lt;StartTmWnd>0000&lt;/StartTmWnd>
                     &lt;EndTmWnd>2359&lt;/EndTmWnd>
                     &lt;JrnyTm/>
                     &lt;FltTypeInd/>
                     &lt;FltTypePref/>
                     &lt;StartPtInd>N&lt;/StartPtInd>
                     &lt;EndPtInd>N&lt;/EndPtInd>
                     &lt;IgnoreTSPref>N&lt;/IgnoreTSPref>
                  &lt;/GenAvail>
               &lt;/AirAvailMods>
               &lt;SuperBBMods>
                  &lt;!--This section checks for fares-->
                 &lt;!-- &lt;FlexShopMods>
                     &lt;ReqType>A&lt;/ReqType>
                     &lt;PremiumInd/>
                     &lt;MoreDaysInd/>
                     &lt;MoreDaysBefore/>
                     &lt;MoreDaysAfter/>
                     &lt;MoreDaysInd2/>
                     &lt;MoreDaysBefore2/>
                     &lt;MoreDaysAfter2/>
                     &lt;ODPricingInd/>
                     &lt;ODDistanceTypeInd/>
                     &lt;ODRadiusDistance/>
                     &lt;ODPricingInd2/>
                     &lt;ODDistanceTypeInd2/>
                     &lt;ODRadiusDistance2/>
                     &lt;AirportOrigin>SAN&lt;/AirportOrigin>
                     &lt;FAOC/>
                     &lt;AirportOrigin2>SFO&lt;/AirportOrigin2>
                     &lt;FAOC2/>
                     &lt;AirportOrigin3>SNA&lt;/AirportOrigin3>
                     &lt;FAOC3/>
                     &lt;AirportOrigin4/>
                     &lt;FAOC4/>
                     &lt;AirportOrigin5/>
                     &lt;FAOC5/>
                     &lt;AirportReturn>FLL&lt;/AirportReturn>
                     &lt;FARC/>
                     &lt;AirportReturn2>TPA&lt;/AirportReturn2>
                     &lt;FARC2/>
                     &lt;AirportReturn3>ATL&lt;/AirportReturn3>
                     &lt;FARC3/>
                     &lt;AirportReturn4/>
                     &lt;FARC4/>
                     &lt;AirportReturn5/>
                     &lt;FARC5/>
                  &lt;/FlexShopMods>-->
                  &lt;ClassPrefs>
                     &lt;!--Required for Class Specific-->
                     &lt;ODPairAry>
                        &lt;ODPair>
                           &lt;!--Required for each ODPair(Outbound/Destination Pair) for Class Specific-->
                           &lt;ODNum>1&lt;/ODNum>
                           &lt;!--Incremental number required to reference 1st OD Pair LAXSYD-->
                           &lt;ClassPref>Y&lt;/ClassPref>
                        &lt;/ODPair>
                        &lt;ODPair>
                           &lt;!--Required for each ODPair(Outbound/Destination Pair) for Class Specific-->
                           &lt;ODNum>2&lt;/ODNum>
                           &lt;!--Incremental number required to reference 2ndOD Pair SYDLAX-->
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
                           &lt;PTC/>
                           &lt;!--Blank defaults to Adult-->
                           &lt;TIC/>
                        &lt;/Psgr>
                        &lt;Psgr>
                           &lt;LNameNum>1&lt;/LNameNum>
                           &lt;PsgrNum>2&lt;/PsgrNum>
                           &lt;AbsNameNum>2&lt;/AbsNameNum>
                           &lt;PTC>C04&lt;/PTC>
                           &lt;!--4 year old Child-->
                           &lt;TIC/>
                        &lt;/Psgr>
                        &lt;Psgr>
                           &lt;LNameNum>1&lt;/LNameNum>
                           &lt;PsgrNum>3&lt;/PsgrNum>
                           &lt;AbsNameNum>3&lt;/AbsNameNum>
                           &lt;PTC>INF&lt;/PTC>
                           &lt;TIC/>
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
                        &lt;KlrID>GFAC&lt;/KlrID>
                     &lt;/KlrIDAry>
                  &lt;/Optimize>
                  &lt;Optimize>
                     &lt;RecType>1001&lt;/RecType>
                     &lt;KlrIDAry>
                        &lt;KlrID>AAFI&lt;/KlrID>
                        &lt;KlrID>AASU&lt;/KlrID>
                        &lt;KlrID>AATC&lt;/KlrID>
                        &lt;KlrID>AATS&lt;/KlrID>
                     &lt;/KlrIDAry>
                  &lt;/Optimize>
                  &lt;Optimize>
                     &lt;RecType>1425&lt;/RecType>
                     &lt;KlrIDAry>
                        &lt;KlrID>GFFC&lt;/KlrID>
                        &lt;KlrID>GFIS&lt;/KlrID>
                        &lt;KlrID>GFMM&lt;/KlrID>
                        &lt;KlrID>GFPF&lt;/KlrID>
                        &lt;KlrID>GFPI&lt;/KlrID>
                        &lt;KlrID>GFPX&lt;/KlrID>
                        &lt;KlrID>GFR1&lt;/KlrID>
                        &lt;KlrID>GFRH&lt;/KlrID>
                        &lt;KlrID>GFRI&lt;/KlrID>
                        &lt;KlrID>GFRR&lt;/KlrID>
                        &lt;KlrID>GFSR&lt;/KlrID>
                        &lt;KlrID>GFSU&lt;/KlrID>
                        &lt;KlrID>GFTS&lt;/KlrID>
                        &lt;KlrID>GFEQ&lt;/KlrID>
                     &lt;/KlrIDAry>
                  &lt;/Optimize>
                  &lt;Optimize>
                     &lt;RecType>1425&lt;/RecType>
                     &lt;KlrIDAry>
                        &lt;KlrID>EROR&lt;/KlrID>
                        &lt;!--Error Information-->
                        &lt;KlrID>GFGQ&lt;/KlrID>
                        &lt;!--GenQuoteDetails-->
                     &lt;/KlrIDAry>
                  &lt;/Optimize>
               &lt;/SuperBBMods>

       &lt;/FareQuoteFlexShop_21>

         &lt;/Request>
         &lt;Filter>
            &lt;_/>
         &lt;/Filter>
      &lt;/SubmitXml>
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
      <defaultValue>GlobalVariable.StartDate</defaultValue>
      <description></description>
      <id>f052b19e-b349-4253-b71f-205240c68041</id>
      <masked>false</masked>
      <name>StartDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.EndDate</defaultValue>
      <description></description>
      <id>deb17cb3-a655-443d-aca2-d376ea7d1f61</id>
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

println(&quot;QuoteNumber : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteFlexShop_21.FareInfo.GenQuoteDetails.QuoteNum')))
println(&quot;QuoteType : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteFlexShop_21.FareInfo.GenQuoteDetails.QuoteType')))
println(&quot;LastTkDt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteFlexShop_21.FareInfo.GenQuoteDetails.LastTkDt')))
println(&quot;QuoteDt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteFlexShop_21.FareInfo.GenQuoteDetails.QuoteDt')))
println(&quot;BaseFareCurrency : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteFlexShop_21.FareInfo.GenQuoteDetails.BaseFareCurrency')))
println(&quot;BaseFareAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteFlexShop_21.FareInfo.GenQuoteDetails.BaseFareAmt')))
println(&quot;EquivCurrency : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteFlexShop_21.FareInfo.GenQuoteDetails.EquivCurrency')))
println(&quot;EquivAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteFlexShop_21.FareInfo.GenQuoteDetails.EquivAmt')))
println(&quot;TotCurrency : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteFlexShop_21.FareInfo.GenQuoteDetails.TotCurrency')))
println(&quot;TotAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteFlexShop_21.FareInfo.GenQuoteDetails.TotAmt')))
println(&quot;TaxInfo : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteFlexShop_21.FareInfo.GenQuoteDetails.TaxDataAry.TaxData.Country')))
println(&quot;TaxAmt : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteFlexShop_21.FareInfo.GenQuoteDetails.TaxDataAry.TaxData.Amt')))</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
