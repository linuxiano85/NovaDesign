// NovaDesign - Calculator Module

// Material calculations for construction professionals

// Concrete Calculator
function calculateConcrete() {
    const length = parseFloat(document.getElementById('concrete-length').value) || 0;
    const width = parseFloat(document.getElementById('concrete-width').value) || 0;
    const thickness = parseFloat(document.getElementById('concrete-thickness').value) || 0;
    
    if (length <= 0 || width <= 0 || thickness <= 0) {
        document.getElementById('concrete-result').innerHTML = 
            '<span style="color: red;">Inserisci valori validi per tutti i campi.</span>';
        return;
    }
    
    // Calculate volume in cubic meters
    const volume = length * width * thickness;
    
    // Concrete typically weighs about 2400 kg/m³
    const weight = volume * 2400;
    
    // Typical concrete mix ratios (per m³):
    // Cement: 350-400 kg
    // Sand: 650-700 kg  
    // Gravel: 1200-1300 kg
    // Water: 175-200 liters
    
    const cement = volume * 375; // kg
    const sand = volume * 675; // kg
    const gravel = volume * 1250; // kg
    const water = volume * 187.5; // liters
    
    // Estimated costs (€/ton or €/m³)
    const cementCost = (cement / 1000) * 120; // €120 per ton
    const sandCost = volume * 25; // €25 per m³
    const gravelCost = volume * 20; // €20 per m³
    const totalCost = cementCost + sandCost + gravelCost;
    
    document.getElementById('concrete-result').innerHTML = `
        <strong>Risultati Calcolatore Cemento:</strong><br>
        <strong>Volume totale:</strong> ${volume.toFixed(2)} m³<br>
        <strong>Peso stimato:</strong> ${(weight/1000).toFixed(2)} tonnellate<br><br>
        <strong>Materiali necessari:</strong><br>
        • Cemento: ${cement.toFixed(0)} kg<br>
        • Sabbia: ${sand.toFixed(0)} kg<br>
        • Ghiaia: ${gravel.toFixed(0)} kg<br>
        • Acqua: ${water.toFixed(0)} litri<br><br>
        <strong>Costo stimato materiali:</strong> €${totalCost.toFixed(2)}
    `;
    
    // Add to activity log
    if (window.addActivity) {
        window.addActivity(`Calcolato cemento: ${volume.toFixed(2)} m³ - €${totalCost.toFixed(2)}`);
    }
}

// Tiles Calculator
function calculateTiles() {
    const area = parseFloat(document.getElementById('tiles-area').value) || 0;
    const tileSize = parseFloat(document.getElementById('tile-size').value) || 30;
    const wastePercent = parseFloat(document.getElementById('tiles-waste').value) || 10;
    
    if (area <= 0) {
        document.getElementById('tiles-result').innerHTML = 
            '<span style="color: red;">Inserisci un\'area valida.</span>';
        return;
    }
    
    // Convert tile size from cm to m
    const tileSizeM = tileSize / 100;
    const tileArea = tileSizeM * tileSizeM; // Area of one tile in m²
    
    // Calculate number of tiles needed
    const tilesNeeded = Math.ceil(area / tileArea);
    
    // Add waste percentage
    const tilesWithWaste = Math.ceil(tilesNeeded * (1 + wastePercent / 100));
    
    // Estimate additional materials
    const adhesiveKg = area * 3; // 3 kg/m² average
    const groutKg = area * 0.5; // 0.5 kg/m² average
    
    // Estimated costs
    const tilesCost = tilesWithWaste * (tileSize <= 20 ? 2 : tileSize <= 40 ? 4 : 8); // Price per tile based on size
    const adhesiveCost = adhesiveKg * 1.5; // €1.5 per kg
    const groutCost = groutKg * 3; // €3 per kg
    const totalCost = tilesCost + adhesiveCost + groutCost;
    
    document.getElementById('tiles-result').innerHTML = `
        <strong>Risultati Calcolatore Piastrelle:</strong><br>
        <strong>Area da rivestire:</strong> ${area} m²<br>
        <strong>Dimensione piastrella:</strong> ${tileSize}x${tileSize} cm<br>
        <strong>Piastrelle necessarie:</strong> ${tilesNeeded} pz<br>
        <strong>Con scarto (${wastePercent}%):</strong> ${tilesWithWaste} pz<br><br>
        <strong>Materiali aggiuntivi:</strong><br>
        • Colla: ${adhesiveKg.toFixed(1)} kg<br>
        • Stucco: ${groutKg.toFixed(1)} kg<br><br>
        <strong>Costo stimato totale:</strong> €${totalCost.toFixed(2)}
    `;
    
    if (window.addActivity) {
        window.addActivity(`Calcolate piastrelle: ${tilesWithWaste} pz per ${area} m² - €${totalCost.toFixed(2)}`);
    }
}

// Paint Calculator
function calculatePaint() {
    const area = parseFloat(document.getElementById('paint-area').value) || 0;
    const coats = parseInt(document.getElementById('paint-coats').value) || 2;
    const coverage = parseFloat(document.getElementById('paint-coverage').value) || 12;
    
    if (area <= 0) {
        document.getElementById('paint-result').innerHTML = 
            '<span style="color: red;">Inserisci un\'area valida.</span>';
        return;
    }
    
    // Calculate total paint needed
    const totalArea = area * coats;
    const paintLiters = totalArea / coverage;
    
    // Round up to nearest 0.5 liters
    const paintLitersRounded = Math.ceil(paintLiters * 2) / 2;
    
    // Additional materials
    const primerLiters = area / (coverage * 1.2); // Primer has better coverage
    const brushes = Math.ceil(area / 50); // 1 brush per 50 m²
    const rollers = Math.ceil(area / 100); // 1 roller per 100 m²
    
    // Estimated costs
    const paintCost = paintLitersRounded * 25; // €25 per liter
    const primerCost = Math.ceil(primerLiters) * 20; // €20 per liter
    const toolsCost = (brushes * 8) + (rollers * 15); // Brushes €8, rollers €15
    const totalCost = paintCost + primerCost + toolsCost;
    
    document.getElementById('paint-result').innerHTML = `
        <strong>Risultati Calcolatore Pittura:</strong><br>
        <strong>Area da dipingere:</strong> ${area} m²<br>
        <strong>Numero di mani:</strong> ${coats}<br>
        <strong>Pittura necessaria:</strong> ${paintLitersRounded} litri<br>
        <strong>Primer consigliato:</strong> ${Math.ceil(primerLiters)} litri<br><br>
        <strong>Strumenti necessari:</strong><br>
        • Pennelli: ${brushes} pz<br>
        • Rulli: ${rollers} pz<br><br>
        <strong>Costo stimato totale:</strong> €${totalCost.toFixed(2)}
    `;
    
    if (window.addActivity) {
        window.addActivity(`Calcolata pittura: ${paintLitersRounded}L per ${area} m² - €${totalCost.toFixed(2)}`);
    }
}

// Electric Installation Calculator
function calculateElectric() {
    const power = parseFloat(document.getElementById('electric-power').value) || 0;
    const cableLength = parseFloat(document.getElementById('cable-length').value) || 0;
    const lightPoints = parseInt(document.getElementById('light-points').value) || 0;
    
    if (power <= 0 && cableLength <= 0 && lightPoints <= 0) {
        document.getElementById('electric-result').innerHTML = 
            '<span style="color: red;">Inserisci almeno un valore valido.</span>';
        return;
    }
    
    // Cable calculations
    let cableSection = 1.5; // mm² default
    if (power > 3) cableSection = 2.5;
    if (power > 6) cableSection = 4;
    if (power > 10) cableSection = 6;
    if (power > 15) cableSection = 10;
    
    // Materials needed
    const cable = cableLength * 3; // 3 cores (phase, neutral, earth)
    const switches = lightPoints;
    const outlets = Math.ceil(power / 2); // Estimate 1 outlet per 2kW
    const breakerSize = Math.ceil(power / 0.23); // Amperes (230V single phase)
    
    // Conduit and accessories
    const conduit = cableLength * 1.1; // 10% extra for bends
    const junction_boxes = Math.ceil(cableLength / 20); // 1 box per 20m
    
    // Estimated costs
    const cableCost = cable * (cableSection * 0.8); // €0.8 per mm² per meter
    const switchesCost = switches * 15; // €15 per switch
    const outletsCost = outlets * 12; // €12 per outlet
    const breakerCost = 35; // €35 per breaker
    const conduitCost = conduit * 2; // €2 per meter
    const accessoriesCost = junction_boxes * 8; // €8 per box
    
    const totalCost = cableCost + switchesCost + outletsCost + breakerCost + conduitCost + accessoriesCost;
    
    document.getElementById('electric-result').innerHTML = `
        <strong>Risultati Calcolatore Elettrico:</strong><br>
        <strong>Potenza totale:</strong> ${power} kW<br>
        <strong>Interruttore consigliato:</strong> ${breakerSize}A<br>
        <strong>Sezione cavo:</strong> ${cableSection} mm²<br><br>
        <strong>Materiali necessari:</strong><br>
        • Cavo: ${cable.toFixed(0)} metri<br>
        • Interruttori: ${switches} pz<br>
        • Prese: ${outlets} pz<br>
        • Tubo corrugato: ${conduit.toFixed(0)} metri<br>
        • Scatole derivazione: ${junction_boxes} pz<br><br>
        <strong>Costo stimato materiali:</strong> €${totalCost.toFixed(2)}<br>
        <span style="font-size: 0.9em; color: #666;">
        *Costi indicativi. Consigliabile consultare un elettricista qualificato.
        </span>
    `;
    
    if (window.addActivity) {
        window.addActivity(`Calcolato impianto elettrico: ${power}kW - €${totalCost.toFixed(2)}`);
    }
}

// Wood Calculator (for carpenters)
function calculateWood() {
    const length = parseFloat(document.getElementById('wood-length').value) || 0;
    const width = parseFloat(document.getElementById('wood-width').value) || 0;
    const thickness = parseFloat(document.getElementById('wood-thickness').value) || 0;
    const pieces = parseInt(document.getElementById('wood-pieces').value) || 1;
    
    if (length <= 0 || width <= 0 || thickness <= 0) {
        return;
    }
    
    // Volume in cubic meters
    const volumePerPiece = (length * width * thickness) / 1000000; // Convert from mm³ to m³
    const totalVolume = volumePerPiece * pieces;
    
    // Board feet calculation (for international reference)
    const boardFeetPerPiece = (length * width * thickness) / 2359737; // Convert to board feet
    const totalBoardFeet = boardFeetPerPiece * pieces;
    
    // Estimated costs (varies greatly by wood type)
    const softWoodCost = totalVolume * 600; // €600 per m³ for softwood
    const hardWoodCost = totalVolume * 1200; // €1200 per m³ for hardwood
    
    // Additional materials
    const screwsBoxes = Math.ceil(pieces / 10); // 1 box per 10 pieces
    const glueBottles = Math.ceil(totalVolume * 2); // Bottles needed
    const sandpaperSheets = Math.ceil(pieces / 2); // Sheets needed
    
    const accessoriesCost = (screwsBoxes * 12) + (glueBottles * 8) + (sandpaperSheets * 3);
    
    return {
        volume: totalVolume,
        boardFeet: totalBoardFeet,
        softWoodCost: softWoodCost + accessoriesCost,
        hardWoodCost: hardWoodCost + accessoriesCost,
        screws: screwsBoxes,
        glue: glueBottles,
        sandpaper: sandpaperSheets
    };
}

// Plumbing Calculator
function calculatePlumbing() {
    const pipeLength = parseFloat(document.getElementById('pipe-length').value) || 0;
    const pipeDiameter = parseFloat(document.getElementById('pipe-diameter').value) || 20;
    const fixtures = parseInt(document.getElementById('fixtures').value) || 0;
    
    if (pipeLength <= 0 && fixtures <= 0) {
        return;
    }
    
    // Pipe calculations
    const elbows = Math.ceil(pipeLength / 5); // 1 elbow per 5 meters average
    const tees = Math.ceil(fixtures / 2); // T-joints for fixtures
    const valves = fixtures; // 1 valve per fixture
    
    // Fittings and accessories
    const couplings = Math.ceil(pipeLength / 6); // Pipe couplings
    const supports = Math.ceil(pipeLength / 2); // Pipe supports every 2m
    
    // Cost calculations (per mm diameter)
    const pipeCostPerMeter = pipeDiameter * 0.1; // €0.1 per mm per meter
    const pipeCost = pipeLength * pipeCostPerMeter;
    const fittingsCost = (elbows * 5) + (tees * 8) + (valves * 15) + (couplings * 3) + (supports * 2);
    
    return {
        pipeLength: pipeLength,
        pipeCost: pipeCost,
        elbows: elbows,
        tees: tees,
        valves: valves,
        totalCost: pipeCost + fittingsCost
    };
}

// Insulation Calculator
function calculateInsulation() {
    const area = parseFloat(document.getElementById('insulation-area').value) || 0;
    const thickness = parseFloat(document.getElementById('insulation-thickness').value) || 50;
    const type = document.getElementById('insulation-type').value || 'rockwool';
    
    if (area <= 0) {
        return;
    }
    
    // Volume of insulation needed
    const volume = area * (thickness / 1000); // Convert mm to meters
    
    // Cost per m³ varies by type
    const costs = {
        rockwool: 45,
        glasswool: 40,
        eps: 30,
        xps: 55,
        polyurethane: 85
    };
    
    const costPerM3 = costs[type] || 45;
    const materialCost = volume * costPerM3;
    
    // Additional materials
    const vaporBarrier = area * 1.2; // 20% overlap
    const fixings = Math.ceil(area * 5); // 5 fixings per m²
    const tape = Math.ceil(area / 20); // Sealing tape rolls
    
    const accessoriesCost = (vaporBarrier * 2) + (fixings * 0.5) + (tape * 8);
    const totalCost = materialCost + accessoriesCost;
    
    return {
        area: area,
        volume: volume,
        materialCost: materialCost,
        accessoriesCost: accessoriesCost,
        totalCost: totalCost,
        vaporBarrier: vaporBarrier,
        fixings: fixings
    };
}

// Export calculator functions for global use
window.calculateConcrete = calculateConcrete;
window.calculateTiles = calculateTiles;
window.calculatePaint = calculatePaint;
window.calculateElectric = calculateElectric;
window.calculateWood = calculateWood;
window.calculatePlumbing = calculatePlumbing;
window.calculateInsulation = calculateInsulation;