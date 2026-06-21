//! Skeletal Vampire — `{4}{B}{B}` 3/3 Vampire Skeleton with Flying.
//! "When this creature enters, create two 1/1 black Bat creature tokens
//! with flying." "{3}{B}{B}, Sacrifice a Bat: Create two 1/1 black Bat
//! creature tokens with flying." "Sacrifice a Bat: Regenerate this
//! creature."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skeletal Vampire");
    let vampire = reg.interner_mut().intern("Vampire");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(skeleton);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let bat = reg.interner_mut().intern("Bat");
    let bat_filter = ObjectFilter::creature().with_subtype_sym(bat);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_two_bats,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}{B}, Sacrifice a Bat: Create two 1/1 black Bat creature tokens with flying.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}{B}").expect("valid cost"),
                    sacrifice_other: Some(bat_filter.clone()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_two_bats_activated,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice a Bat: Regenerate this creature.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(bat_filter.clone()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: regenerate_self,
            }),
    )
}

fn bat_token(reg: &CardRegistry) -> TokenDefinition {
    let bat = reg.interner().lookup("Bat").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);
    TokenDefinition {
        name: bat,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    }
}

fn etb_make_two_bats(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::CreateToken { controller: trig.controller, token: bat_token(reg) },
        Effect::CreateToken { controller: trig.controller, token: bat_token(reg) },
    ]
}

fn make_two_bats_activated(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::CreateToken { controller: ctx.controller, token: bat_token(reg) },
        Effect::CreateToken { controller: ctx.controller, token: bat_token(reg) },
    ]
}

fn regenerate_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Regenerate { target: ctx.source }]
}
