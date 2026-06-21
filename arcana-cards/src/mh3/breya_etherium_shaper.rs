//! Breya, Etherium Shaper — `{W}{U}{B}{R}` 4/4 Legendary Artifact Creature
//! — Human. "When Breya enters, create two 1/1 blue Thopter artifact
//! creature tokens with flying." `{2}, Sacrifice two artifacts: Choose
//! one — deal 3 damage to a player/PW; or -4/-4 a creature; or gain 5
//! life." Activated abilities have no modal machinery in the demonstrated
//! API, so the activation cost is faithful and the resolver implements the
//! deterministic "gain 5 life" mode; the other two modes are GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
use arcana_core::effects::TokenDefinition;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Breya, Etherium Shaper");
    let human = reg.interner_mut().intern("Human");
    let _thopter = reg.interner_mut().intern("Thopter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
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
                effect: etb_make_thopters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, Sacrifice two artifacts: Choose one — Breya deals 3 damage to target player or planeswalker; or target creature gets -4/-4 until end of turn; or you gain 5 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    sacrifice_other: Some(
                        ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into()),
                    ),
                    sacrifice_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: modal_gain_life,
            }),
    )
}

fn etb_make_thopters(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let thopter = reg.interner().lookup("Thopter").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thopter);
    let token = TokenDefinition {
        name: thopter,
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}

fn modal_gain_life(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal activated ability ("Choose one — …") — activated abilities
    // have no modal dispatch in the demonstrated API. Implementing the
    // deterministic targetless "you gain 5 life" mode; the "3 damage to a
    // player/planeswalker" and "-4/-4 a creature" modes are not selectable.
    vec![Effect::GainLife { player: ctx.controller, amount: 5 }]
}
