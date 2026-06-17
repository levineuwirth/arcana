//! Sefris of the Hidden Ways — {W}{U}{B} 2/3 Legendary Creature — Human Wizard.
//! Whenever one or more creature cards are put into your graveyard from
//! anywhere, venture into the dungeon. This ability triggers only once each
//! turn.
//! Create Undead — Whenever you complete a dungeon, return target creature
//! card from your graveyard to the battlefield (GAP — no "complete a dungeon"
//! TriggerCondition variant exists).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sefris of the Hidden Ways");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "Venture into the dungeon" is a mechanic, not a KeywordAbility.
        keywords: vec![],
        ..Default::default()
    };
    // GAP: "Whenever you complete a dungeon, return target creature card from
    // your graveyard to the battlefield" — no dungeon-completion
    // TriggerCondition variant exists; the second ability is omitted.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature(),
                from: None,
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: venture,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::OncePerTurn,
            target_requirements: Vec::new(),
        }),
    )
}

fn venture(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Venture { player: trig.controller }]
}
