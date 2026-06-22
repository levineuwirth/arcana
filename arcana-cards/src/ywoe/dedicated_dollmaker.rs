//! Dedicated Dollmaker — `{1}{W}` 2/2 Dwarf Artificer (white).
//! When Dedicated Dollmaker enters, exile up to one other target nonland,
//! nontoken permanent. Its controller creates a token that's a copy of it,
//! except it's not legendary and it's an artifact in addition to its other
//! types.
//! {1}{W}: Tokens you control gain indestructible until end of turn. Activate
//! only once.
//!
//! The ETB targets up to one nonland, nontoken permanent. The full "exile, then
//! its controller creates a modified (non-legendary, artifact-added) token
//! copy" is not expressible — CopyPermanent has no controller override and no
//! modification riders, and exiling first removes the copy source — so the ETB
//! body is GAP'd while the target is still declared. The activated ability
//! grants indestructible to every token you control via ForEach.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dedicated Dollmaker");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .without_types(TypeLine::LAND.into())
                            .nontoken(),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            .with_activated_ability(ActivatedAbilityDef {
                // GAP (fidelity): "Activate only once" approximated by once_per_turn (nearest available gate).
                text: "{1}{W}: Tokens you control gain indestructible until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}").expect("valid cost"),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tokens_indestructible,
            }),
    )
}

fn etb_copy(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "exile … its controller creates a token that's a copy of it, except
    // it's not legendary and an artifact" — CopyPermanent has no controller
    // override or modification riders; the exile-then-copy ordering is also
    // not expressible.
    Vec::new()
}

fn tokens_indestructible(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::permanent()
            .controlled_by(ControllerConstraint::You)
            .tokens_only(),
        ctx.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::GrantKeyword {
            target: NULL_OBJECT_ID,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        }),
    }]
}
