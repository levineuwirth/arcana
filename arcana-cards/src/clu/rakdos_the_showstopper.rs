//! Rakdos, the Showstopper — `{4}{B}{R}` 6/6 Legendary Demon with Flying and
//! Trample.
//! "When Rakdos enters, flip a coin for each creature that isn't a Demon,
//!  Devil, or Imp. Destroy each creature whose coin comes up tails."
//!
//! The ETB enumerates every creature that is not a Demon/Devil/Imp and, per
//! creature, flips a coin: on a loss the creature is destroyed (win is a no-op).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rakdos, the Showstopper");
    let demon = reg.interner_mut().intern("Demon");
    let _devil = reg.interner_mut().intern("Devil");
    let _imp = reg.interner_mut().intern("Imp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_coin_massacre,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_coin_massacre(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let mut filter = ObjectFilter::creature();
    if let Some(demon) = reg.interner().lookup("Demon") {
        filter = filter.without_subtype_sym(demon);
    }
    if let Some(devil) = reg.interner().lookup("Devil") {
        filter = filter.without_subtype_sym(devil);
    }
    if let Some(imp) = reg.interner().lookup("Imp") {
        filter = filter.without_subtype_sym(imp);
    }
    let ids = script::ids_matching(state, &filter, trig.controller);
    ids.into_iter()
        .map(|id| Effect::FlipCoin {
            player: trig.controller,
            win: Box::new(Effect::Sequence(vec![])),
            lose: Some(Box::new(Effect::DestroyPermanent { target: id })),
        })
        .collect()
}
