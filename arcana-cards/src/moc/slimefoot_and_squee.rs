//! Slimefoot and Squee — `{B}{R}{G}` 3/3 Legendary Fungus Goblin.
//! "Whenever Slimefoot and Squee enters or attacks, create a 1/1 green
//!  Saproling creature token.
//!  {1}{B}{R}{G}, Sacrifice a Saproling: Return this card and up to one other
//!  target creature card from your graveyard to the battlefield. Activate only
//!  as a sorcery."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slimefoot and Squee");
    let fungus = reg.interner_mut().intern("Fungus");
    let goblin = reg.interner_mut().intern("Goblin");
    reg.interner_mut().intern("Saproling");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    subtypes.0.insert(goblin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let saproling_cost_filter = script::subtype_filter(reg, "Saproling");

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_saproling,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: make_saproling,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}{R}{G}, Sacrifice a Saproling: Return this card and up to one other target creature card from your graveyard to the battlefield. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}{R}{G}").expect("valid cost"),
                    sacrifice_other: Some(saproling_cost_filter),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: arcana_core::targets::ObjectFilter::creature(),
                    },
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: reanimate,
            }),
    )
}

fn make_saproling(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let saproling = reg.interner().lookup("Saproling").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saproling);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: saproling,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn reanimate(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }];
    if let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() {
        effects.push(Effect::ReturnFromGraveyardToBattlefield { target: *id });
    }
    effects
}
