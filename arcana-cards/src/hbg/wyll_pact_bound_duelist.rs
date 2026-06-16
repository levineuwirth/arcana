//! Wyll, Pact-Bound Duelist — `{3}{R}{R}` 4/4 Legendary Human Warlock (red).
//!
//! Oracle:
//! * "Specialize {2}" — Scryfall keyword `Specialize`. There is no
//!   `KeywordAbility::Specialize` and the specialize ACTIVATION (a paid
//!   colored-back transform, CR 711) is not expressible on this card class
//!   (no specialize cost field / activated-ability form). GAP'd: the
//!   keyword is recorded only via this note. (`SelfSpecializes` is a
//!   trigger condition, not the activation itself.)
//! * "When Wyll enters, gain control of target artifact or creature an
//!   opponent controls with mana value 4 or less until the end of your
//!   next turn." — ETB trigger targeting an opponent-controlled
//!   artifact-or-creature with mv ≤ 4. The control change is modeled with
//!   `ChangeControlEot` (until end of turn); the "until the end of your
//!   NEXT turn" duration is a fidelity gap — the engine reverts at the
//!   next end step rather than at the end of your next turn.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wyll, Pact-Bound Duelist");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_steal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
                            .controlled_by(ControllerConstraint::Opponent)
                            .with_max_cmc(4),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_steal(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ChangeControlEot { target: *id, new_controller: trig.controller }]
}
