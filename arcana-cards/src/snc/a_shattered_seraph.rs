//! A-Shattered Seraph — `{4}{W}{U}{B}` 4/5 white/blue/black Angel
//! Rogue with Flying.
//!
//! * Flying.
//! * "When Shattered Seraph enters, you gain 3 life." — implemented.
//! * "{1}, Exile Shattered Seraph from your hand: Target land gains
//!   '{T}: Add {W}, {U}, or {B}' until Shattered Seraph is cast from
//!   exile. You may cast Shattered Seraph for as long as it remains
//!   exiled." — the activation cost (mana + exile-self from hand) is
//!   expressed, but the EFFECT (granting the land a temporary mana
//!   ability gated on "until ~ is cast from exile", plus the cast-
//!   from-exile permission) is GAP'd: no Effect grants a conditional
//!   activated ability tied to a cast-from-exile window, and there is
//!   no cast-while-exiled permission primitive.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Shattered Seraph");
    let angel = reg.interner_mut().intern("Angel");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "When Shattered Seraph enters, you gain 3 life."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_gain_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "{1}, Exile ~ from your hand: Target land gains ... ."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Exile Shattered Seraph from your hand: Target land \
                       gains \"{T}: Add {W}, {U}, or {B}\" until Shattered \
                       Seraph is cast from exile."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").unwrap(),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Hand,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_land_mana_ability,
            }),
    )
}

fn etb_gain_three(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 3,
    }]
}

fn grant_land_mana_ability(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: granting the targeted land a temporary "{T}: Add {W}, {U}, or
    // {B}" mana ability gated on "until ~ is cast from exile", plus the
    // cast-from-exile permission, is not expressible with the available
    // Effect catalog.
    Vec::new()
}
