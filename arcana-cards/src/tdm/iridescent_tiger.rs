//! Iridescent Tiger — `{4}{R}` 3/4 red Cat creature. "When this
//! creature enters, if you cast it, add {W}{U}{B}{R}{G}." Models the
//! ETB trigger and the five-color mana production; the "if you cast
//! it" intervening clause is GAPped (no engine surface for cast-vs-
//! put-onto-battlefield discrimination as an intervening-if).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Iridescent Tiger");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: intervening "if you cast it" clause — no engine
                // surface to gate an intervening-if on cast-vs-put-onto-
                // battlefield. Trigger fires on any ETB.
                intervening_if: None,
                effect: etb_add_wubrg,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: add one mana of each color to the controller's pool.
fn etb_add_wubrg(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mana = vec![
        ManaUnit::plain(ManaColor::White, trig.source),
        ManaUnit::plain(ManaColor::Blue, trig.source),
        ManaUnit::plain(ManaColor::Black, trig.source),
        ManaUnit::plain(ManaColor::Red, trig.source),
        ManaUnit::plain(ManaColor::Green, trig.source),
    ];
    vec![Effect::AddMana { player: trig.controller, mana }]
}
