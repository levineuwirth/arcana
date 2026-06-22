//! Hunted Horror — `{B}{B}` 7/7 Horror with Trample.
//!
//! * Trample (keyword).
//! * "When this creature enters, target opponent creates two 3/3 green
//!   Centaur creature tokens with protection from black." — a
//!   `SelfEntersBattlefield` trigger targeting an opponent (player)
//!   that mints two 3/3 green Centaur tokens under that player's
//!   control. GAP: the "protection from black" rider on the tokens —
//!   Protection is not in the usable keyword set for this card class,
//!   so the tokens are minted without it.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
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
    let name = reg.interner_mut().intern("Hunted Horror");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);
    // Pre-intern the token subtype so the resolver can rebuild it.
    let _centaur = reg.interner_mut().intern("Centaur");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: opponent_makes_centaurs,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Player,
                count: TargetCount::Exactly(1),
                controller: Some(ControllerConstraint::Opponent),
            }],
        }),
    )
}

fn opponent_makes_centaurs(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    let centaur = reg.interner().lookup("Centaur").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    // GAP: "with protection from black" — Protection not in usable keyword set.
    let token = TokenDefinition {
        name: centaur,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: *p, token: token.clone() },
        Effect::CreateToken { controller: *p, token },
    ]
}
