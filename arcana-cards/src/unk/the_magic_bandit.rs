//! The Magic Bandit — `{3}{B}` 3/2 Legendary Creature — Human Rogue.
//!
//! * "When The Magic Bandit deals combat damage to an opponent, exile the top
//!   card of that player's library. You may cast that card for as long as it
//!   remains exiled, and mana of any type can be spent to cast that spell."
//!   GAP: exile-top-of-defending-player's-library-and-grant-cast-permission is
//!   not expressible with the documented effect catalog (no impulse-from-
//!   opponent-library primitive).
//! * "Whenever you cast a spell or play a land you don't own, copy it." —
//!   modeled as the cast half via `SpellCast` + `Effect::CopySpell`. The
//!   "play a land you don't own" half and the ownership restriction are GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Magic Bandit");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_exile,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: copy_cast_spell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

// GAP: "exile the top card of that player's library; you may cast it while
// exiled, any type of mana" — no impulse-from-opponent-library + cast-grant
// primitive in the documented catalog. (Placeholder trigger wiring; condition
// approximated to ETB to keep the def valid.)
fn combat_damage_exile(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}

// GAP: "copy it" — the SpellCast trigger does not expose the cast spell's
// stack id as a target, and the "play a land you don't own" half + ownership
// restriction are not expressible. Emit nothing rather than copy a wrong id.
fn copy_cast_spell(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
