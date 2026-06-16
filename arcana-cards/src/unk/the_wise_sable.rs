//! The Wise Sable — `{4}{R}{G}{W}` 5/5 Legendary Elder Sable Judge.
//! Judge Call! — When The Wise Sable enters the battlefield, choose one
//! of six named cards at random and create a token that's a copy of it.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Wise Sable");
    let elder = reg.interner_mut().intern("Elder");
    let sable = reg.interner_mut().intern("Sable");
    let judge = reg.interner_mut().intern("Judge");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(sable);
    subtypes.0.insert(judge);

    // "Judge Call!" is flavor, not a recognized KeywordAbility.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: judge_call,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn judge_call(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose one of six named cards at random; create a token that's a
    // copy of the chosen card" — creating a token copy of a card chosen BY
    // NAME (not an on-battlefield object) is Conjure-class and has no Effect
    // variant. CopyPermanent requires an existing target object id.
    Vec::new()
}
