<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>FareQuoteMultiDisplay</name>
   <tag></tag>
   <elementGuidId>52c25eb2-69cf-411c-9f2b-d1c19e109a45</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
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
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:web=&quot;http://webservices.galileo.com&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;web:SubmitXml>
         &lt;web:Profile>${Profile}&lt;/web:Profile>
         &lt;web:Request>  
&lt;FareQuoteMultiDisplay_31>
	&lt;FareDisplayMods>
		&lt;QueryHeader>
			&lt;UniqueKey>0000&lt;/UniqueKey>
			&lt;LangNum>00&lt;/LangNum>
			&lt;Action>002&lt;/Action>
			&lt;RetCRTOutput>N&lt;/RetCRTOutput>
			&lt;NoMsg>N&lt;/NoMsg>
			&lt;NoTrunc>N&lt;/NoTrunc>
			&lt;IMInd>N&lt;/IMInd>
			&lt;FIPlus>N&lt;/FIPlus>
			&lt;PEInd>N&lt;/PEInd>
			&lt;PVInd>N&lt;/PVInd>
			&lt;NBInd>N&lt;/NBInd>
			&lt;ActionOnlyInd>N&lt;/ActionOnlyInd>
			&lt;TranslatePeriod>N&lt;/TranslatePeriod>
			&lt;PIInd>N&lt;/PIInd>
			&lt;IntFrame1>N&lt;/IntFrame1>
			&lt;SmartParsed>N&lt;/SmartParsed>
			&lt;PDCodes>N&lt;/PDCodes>
			&lt;BkDtOverride>N&lt;/BkDtOverride>
			&lt;HostUse25>N&lt;/HostUse25>
			&lt;DefCurrency>N&lt;/DefCurrency>
			&lt;PFPWInd>N&lt;/PFPWInd>
			&lt;PFPQInd>N&lt;/PFPQInd>
			&lt;HostUse29>N&lt;/HostUse29>
			&lt;HostUse30>N&lt;/HostUse30>
			&lt;HostUse31>N&lt;/HostUse31>
			&lt;HostUse32>N&lt;/HostUse32>
			&lt;HostUse33>N&lt;/HostUse33>
		&lt;/QueryHeader>
		&lt;TravConstraints>
			&lt;UniqueKey>0000&lt;/UniqueKey>
			&lt;StartPt>${Origin}&lt;/StartPt>
			&lt;EndPt>${Destination}&lt;/EndPt>
			&lt;OW>N&lt;/OW>
			&lt;RT>N&lt;/RT>
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
			&lt;CalShopReq>&lt;/CalShopReq>
			&lt;CheckAltCitiesExist>N&lt;/CheckAltCitiesExist>
			&lt;RetAltCityQuote>N&lt;/RetAltCityQuote>
			&lt;StartDt>${StartDate}&lt;/StartDt>
			&lt;AirV1>&lt;/AirV1>
			&lt;AirV2>&lt;/AirV2>
			&lt;AirV3>&lt;/AirV3>
			&lt;GlobDir>&lt;/GlobDir>
			&lt;ConxPt1>&lt;/ConxPt1>
			&lt;ConxPt2>&lt;/ConxPt2>
			&lt;EndDt/>
			&lt;TkDt/>
			&lt;FareType>&lt;/FareType>
			&lt;Currency>&lt;/Currency>
			&lt;Pt>&lt;/Pt>
			&lt;SellCurrency>&lt;/SellCurrency>
			&lt;JointFares>N&lt;/JointFares>
			&lt;RndWorld>N&lt;/RndWorld>
			&lt;CircTrip>N&lt;/CircTrip>
			&lt;DoubleOneWay>&lt;/DoubleOneWay>
			&lt;AltDatesReq>&lt;/AltDatesReq>
			&lt;Surcharges>N&lt;/Surcharges>
			&lt;Spare1>N&lt;/Spare1>
			&lt;SkipEffDtProcess>&lt;/SkipEffDtProcess>
			&lt;CabinNum>&lt;/CabinNum>
			&lt;EffStartDtFilter>&lt;/EffStartDtFilter>
			&lt;EffEndDtFilter>&lt;/EffEndDtFilter>
		&lt;/TravConstraints>
	&lt;/FareDisplayMods>
&lt;/FareQuoteMultiDisplay_31>
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
   <socketTimeout>0</socketTimeout>
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
      <defaultValue>GlobalVariable.Profile</defaultValue>
      <description></description>
      <id>7d31ecb4-6ef5-4cb1-a8c8-897893780abb</id>
      <masked>false</masked>
      <name>Profile</name>
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

println(&quot;OutputMsg : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.FareQuoteMultiDisplay_31.FareInfo.OutputMsg.Text')))</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
