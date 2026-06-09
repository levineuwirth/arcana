//! Brotherhood Spy — `{1}{U}` 1/3 blue Human Assassin.
//! "At the beginning of combat on your turn, if you control a legendary Assassin, this
//! creature gets +1/+0 until end of turn. It can't be blocked this turn."
//! "if you control a legendary Assassin" intervening-if wired via a legendary-supertype
//! + Assassin-subtype filter on conditions::you_control_a.
//! GAP: "can't be blocked" — unblockable effect not in catalog; emitting pump unconditionally.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::ObjectId;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brotherhood Spy");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
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
                intervening_if: Some(iif_control_legendary_assassin),
                effect: pump_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "if you control a legendary Assassin" — legendary supertype + Assassin subtype.
fn iif_control_legendary_assassin(
    state: &GameState,
    _source: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    match reg.interner().lookup("Assassin") {
        Some(a) => conditions::you_control_a(
            state,
            you,
            &ObjectFilter::new()
                .with_supertypes(SupertypeSet(SupertypeSet::LEGENDARY))
                .with_subtype_sym(a),
        ),
        None => false,
    }
}

fn pump_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "can't be blocked" — unblockable effect not in catalog
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
