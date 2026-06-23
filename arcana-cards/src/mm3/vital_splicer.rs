//! Vital Splicer — `{3}{G}` 1/1 green Phyrexian Human Artificer.
//! "When this creature enters, create a 3/3 colorless Phyrexian Golem
//! artifact creature token."
//! "{1}: Regenerate target Golem you control."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vital Splicer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    // Token subtypes (pre-interned for the token recipe).
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };
    let _ = golem;

    // "Golem" is already interned above, so subtype_filter resolves it.
    let golem_filter =
        arcana_core::script::subtype_filter(reg, "Golem").controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            // "When this creature enters, create a 3/3 colorless Phyrexian
            // Golem artifact creature token."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_golem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "{1}: Regenerate target Golem you control."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: Regenerate target Golem you control.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(golem_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: regenerate_golem,
            }),
    )
}

fn etb_make_golem(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let golem = reg.interner().lookup("Golem").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: golem,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn regenerate_golem(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Regenerate { target: *id }]
}
