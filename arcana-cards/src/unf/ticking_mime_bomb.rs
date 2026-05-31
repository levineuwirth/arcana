//! Ticking Mime Bomb — `{3}{R}` 2/2 red Artifact Creature — Clown Robot
//! Mime. "When this creature enters, ... [a person outside the game]
//! chooses a creature you don't control. This creature deals damage equal
//! to twice the number of Robots you control to the chosen creature."
//!
//! The Un-set "person outside the game pantomimes" framing is flavor for an
//! opponent-style choice; modeled here as a targeted ETB trigger choosing a
//! creature the controller doesn't control, dealing 2 × (Robots you control)
//! damage to it.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ticking Mime Bomb");
    let clown = reg.interner_mut().intern("Clown");
    let robot = reg.interner_mut().intern("Robot");
    let mime = reg.interner_mut().intern("Mime");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(clown);
    subtypes.0.insert(robot);
    subtypes.0.insert(mime);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_pantomime_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        arcana_core::targets::ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_pantomime_damage(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let robots = script::count_matching(
        state,
        &script::subtype_filter(reg, "Robot").controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let amount = robots.saturating_mul(2);
    vec![Effect::DealDamage {
        target: DamageTarget::Object(*id),
        amount,
        source: trig.source,
    }]
}
