//! Mishra, Eminent One — `{2}{U}{B}{R}` 5/4 legendary Human
//! Artificer. "At the beginning of combat on your turn, create a
//! token that's a copy of target noncreature artifact you control,
//! except its name is Mishra's Warform and it's a 4/4 Construct
//! artifact creature in addition to its other types. It gains haste
//! until end of turn. Sacrifice it at the beginning of the next end
//! step."
//!
//! GAP: only the bare copy is expressible — `Effect::CopyPermanent`
//! creates a token copy of the target, but the engine's catalog has
//! no way to mutate the resulting token's name, force-add
//! `Creature — Construct`, override its P/T to 4/4, grant haste to
//! it, or schedule a delayed sacrifice on the freshly-minted token
//! (its ObjectId is not visible to this resolver). The Warform
//! rider is left unimplemented.

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
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mishra, Eminent One");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: warform_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::ARTIFACT.into())
                            .without_types(TypeLine::CREATURE.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn warform_copy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // Best-effort: produce the copy. The Warform rider (rename,
    // 4/4 Construct artifact creature overlay, haste, sac at next
    // end step) is GAP — the engine has no hook to mutate or
    // address the newly-created token here.
    vec![Effect::CopyPermanent { target: *id }]
}
