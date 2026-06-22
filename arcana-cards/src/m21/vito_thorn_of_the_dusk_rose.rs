//! Vito, Thorn of the Dusk Rose — `{2}{B}` 1/3 Legendary Vampire Cleric.
//!
//! Oracle:
//! * Whenever you gain life, target opponent loses that much life.
//! * {3}{B}{B}: Creatures you control gain lifelink until end of turn.
//!
//! The first trigger fires on `LifeGained` (you) and targets an
//! opponent, but the dynamic "that much life" amount is GAP'd — no
//! accessor exposes the amount of life gained by the triggering event
//! (same posture as Defiant Bloodlord / Enduring Tenacity in the
//! catalog). The activated ability grants Lifelink to every creature
//! you control via `ForEach` over the matching ids.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vito, Thorn of the Dusk Rose");
    let vampire = reg.interner_mut().intern("Vampire");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::LifeGained {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: opponent_loses_that_much,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}{B}: Creatures you control gain lifelink until end of turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_lifelink,
            }),
    )
}

fn opponent_loses_that_much(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "loses that much life" — no accessor exposes the amount of life
    // gained by the triggering event, so the dynamic loss amount is uncomputable.
    Vec::new()
}

fn grant_lifelink(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::GrantKeyword {
            target: NULL_OBJECT_ID,
            keyword: KeywordAbility::Lifelink,
            duration: Duration::EndOfTurn,
        }),
    }]
}
