//! Euru, Acorn Scrounger — `{2}{B}{G}` 3/3 Legendary Squirrel Soldier.
//! Forage and Conjure are not in the usable keyword surface (GAP — no
//! KeywordAbility variants for them).
//! 1. ETB "you may forage. When you do, conjure Chitterspitter" — Forage
//!    and Conjure are unmodeled; effect GAP'd.
//! 2. "Whenever one or more Squirrels you control deal combat damage to a
//!    player, you may sacrifice a token; if you do, put an acorn counter on
//!    each permanent you control named Chitterspitter." — SacrificeFilter has
//!    no token-only class, so the "sacrifice a token" payment can't be
//!    expressed; effect GAP'd, trigger shell wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Euru, Acorn Scrounger");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: keywords Forage / Conjure have no usable KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    let squirrel_filter = arcana_core::script::subtype_filter(reg, "Squirrel")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_forage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: squirrel_filter,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: squirrels_combat_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_forage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Forage (an optional non-mana cost) and Conjure (Arena-only,
    // create-card-by-name) are both unmodeled.
    Vec::new()
}

fn squirrels_combat_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice a token" — SacrificeFilter has no token-only
    // class, so the payment can't be expressed; the conditional acorn-counter
    // placement cannot be gated; whole effect GAP'd.
    Vec::new()
}
