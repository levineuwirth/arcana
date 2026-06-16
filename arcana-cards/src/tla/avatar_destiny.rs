//! Avatar Destiny — `{2}{G}{G}` enchantment — Aura.
//! "Enchant creature you control. Enchanted creature gets +1/+1 for each
//!  creature card in your graveyard and is an Avatar in addition to its
//!  other types. When enchanted creature dies, mill cards equal to its
//!  power. Return this card to its owner's hand and up to one creature card
//!  milled this way to the battlefield under your control."
//!
//! The dynamic +X/+X (X = creature cards in your graveyard, a type-only
//! filter) and the Avatar subtype add are installed. The host-death
//! mill/return payoff is multi-step and not expressible — GAP.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Avatar Destiny");
    let aura = reg.interner_mut().intern("Aura");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    // Capture the Avatar symbol so the effect fn can install it without reg.
    let _ = avatar;
    reg.register(
        CardDefinition::new(name, chars)
            // NOTE: "you control" approximated by caster's choice.
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "when enchanted creature dies, mill cards equal to its power;
    // return this card to hand and up to one creature card milled this way
    // to the battlefield" — multi-step host-death payoff not expressible.
    let avatar = reg.interner().lookup("Avatar").expect("Avatar interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt_dynamic(
                trig.source,
                creatures_in_your_graveyard,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_subtypes(
                trig.source,
                subtypes,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

fn creatures_in_your_graveyard(state: &GameState, source: ObjectId) -> (i32, i32) {
    let you = state.object_or_lki(source).map(|o| o.controller).unwrap_or(0);
    let total = script::graveyard_matching(
        state,
        &ObjectFilter::creature(),
        you,
        you,
    );
    (total as i32, total as i32)
}
