//! Aphelia, Viper Whisperer — `{1}{B}` 1/3 Legendary Gorgon Assassin with Deathtouch.
//! "Whenever Aphelia attacks, you may pay {1}{B/G}. If you do, create a 1/1
//!  black Snake creature token with deathtouch."
//! "{4}{B}: Until end of turn, whenever one or more Gorgons and/or Snakes you
//!  control deal combat damage to a player, that player loses half their life,
//!  rounded up."

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aphelia, Viper Whisperer");
    let gorgon = reg.interner_mut().intern("Gorgon");
    let assassin = reg.interner_mut().intern("Assassin");
    let _snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gorgon);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_make_snake,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: "{4}{B}: Until end of turn, whenever Gorgons/Snakes you control
        // deal combat damage to a player, that player loses half their life
        // rounded up" — a floating delayed grant whose payoff is a dynamic
        // half-life-rounded-up loss; the half-rounding amount is not
        // computable with the demonstrated script helpers.
    )
}

fn attack_make_snake(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let snake = reg.interner().lookup("Snake").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}{B/G}").expect("valid cost")),
        then: Box::new(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: snake,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Deathtouch],
                abilities: vec![],
            },
        }),
        else_effect: None,
    }]
}
