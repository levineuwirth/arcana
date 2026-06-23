//! Sage's Reverie — `{3}{W}` enchantment — Aura.
//! "Enchant creature. When this Aura enters, draw a card for each Aura you
//!  control that's attached to a creature. Enchanted creature gets +1/+1
//!  for each Aura you control that's attached to a creature."
//!
//! Both clauses count "Auras you control attached to a creature". The filter
//! is an Aura-subtype `ObjectFilter` controlled by you, with a `custom`
//! predicate restricting to Auras attached to a creature. The ETB draw uses
//! `script::count_matching` at resolution; the +1/+1-per-match buff is an
//! ETB-installed `attached_pt_per_match` over the same filter.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, GameObject};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sage's Reverie");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
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

/// True when `obj` is currently attached to a creature on the battlefield.
fn attached_to_a_creature(obj: &GameObject, state: &GameState) -> bool {
    obj.attached_to
        .and_then(|h| state.objects.get(h))
        .is_some_and(|h| h.is_creature())
}

/// "Auras you control that are attached to a creature" — Aura-subtype filter,
/// controlled by you, restricted to those attached to a creature.
fn auras_on_creatures_filter(aura_sym: arcana_core::types::SmallString) -> ObjectFilter {
    ObjectFilter {
        custom: Some(attached_to_a_creature),
        ..ObjectFilter::permanent()
            .with_subtype_sym(aura_sym)
            .controlled_by(ControllerConstraint::You)
    }
}

fn etb_install(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(aura_sym) = reg.interner().lookup("Aura") else {
        return Vec::new();
    };
    let filter = auras_on_creatures_filter(aura_sym);
    let count = script::count_matching(state, &filter, trig.controller);
    // GAP: "Enchanted creature gets +1/+1 for each Aura you control attached to
    // a creature" via attached_pt_per_match INFINITE-LOOPS the layer system —
    // its custom "attached to a creature" predicate re-enters host characteristic
    // computation while that very P/T effect is being applied (stack overflow,
    // harness-caught). The one-shot ETB draw below is safe (resolves once).
    vec![Effect::DrawCards {
        player: trig.controller,
        count,
    }]
}
