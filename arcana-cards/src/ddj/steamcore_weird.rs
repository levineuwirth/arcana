//! Steamcore Weird — `{3}{U}` 1/3 blue Weird. "When this creature
//! enters, if {R} was spent to cast it, it deals 2 damage to any
//! target."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Steamcore Weird");
    let weird = reg.interner_mut().intern("Weird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(weird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
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
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: intervening-if "if {R} was spent to cast it" — no
                // conditions:: predicate tracks which mana was spent on the
                // casting cost, so the {R}-spent gate cannot be expressed.
                intervening_if: None,
                effect: deal_two_any_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }),
    )
}

fn deal_two_any_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let dmg_target = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(_) => match target {
            TargetChoice::ObjectOrPlayer(_) => return Vec::new(),
            _ => return Vec::new(),
        },
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        target: dmg_target,
        amount: 2,
        source: trig.source,
    }]
}
