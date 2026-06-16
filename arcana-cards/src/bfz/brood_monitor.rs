//! Brood Monitor — `{4}{G}{G}` 3/3 Eldrazi Drone, Devoid (colorless).
//! ETB: create three 1/1 colorless Eldrazi Scion creature tokens with
//! "Sacrifice this token: Add {C}."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brood Monitor");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    // Pre-intern the token's subtype so the resolver's read-only lookup
    // ("Scion") succeeds.
    let _scion = reg.interner_mut().intern("Scion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_scions,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_scions(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let eldrazi = reg.interner().lookup("Eldrazi").unwrap_or_default();
    let scion = reg.interner().lookup("Scion").unwrap_or_default();
    // GAP: the tokens' "Sacrifice this token: Add {C}" mana ability is not
    // authored on the TokenDefinition (token abilities left empty); bare 1/1
    // colorless Eldrazi Scion bodies are created.
    let make = || {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(eldrazi);
        subtypes.0.insert(scion);
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: eldrazi,
                colors: ColorSet::colorless(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        }
    };
    vec![make(), make(), make()]
}
