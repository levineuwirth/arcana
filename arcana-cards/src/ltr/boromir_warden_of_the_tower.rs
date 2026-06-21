//! Boromir, Warden of the Tower — `{2}{W}` Legendary 3/3 Human Soldier.
//!
//! Rules text:
//! * Vigilance
//! * Whenever an opponent casts a spell, if no mana was spent to cast it,
//!   counter that spell.
//! * Sacrifice Boromir: Creatures you control gain indestructible until end
//!   of turn. The Ring tempts you.
//!
//! The vigilance keyword and the sacrifice activated ability (board-wide
//! indestructible grant) are faithful. The opponent-cast trigger is GAP'd: the
//! "counter that spell" payload has no accessor for the triggering spell's
//! stack id, and the "if no mana was spent" intervening-if has no predicate.
//! "The Ring tempts you" has no catalog Effect.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boromir, Warden of the Tower");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: counter_free_spell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice Boromir: Creatures you control gain indestructible until end of turn. The Ring tempts you.".into(),
                cost: ActivationCost {
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_indestructible,
            }),
    )
}

fn counter_free_spell(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if no mana was spent to cast it, counter that spell" — there is no
    // intervening-if predicate for "no mana spent", and no accessor exposing the
    // triggering spell's stack-object id to pass to Effect::Counter. Whole
    // trigger effect omitted rather than counter every opponent spell.
    Vec::new()
}

fn grant_indestructible(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "The Ring tempts you" has no catalog Effect.
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        .map(|id| Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        })
        .collect()
}
