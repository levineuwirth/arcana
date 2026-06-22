//! Gadget Technician — `{2}{U}{R}` 3/2 Goblin Artificer.
//! When this creature enters or is turned face up, create a 1/1 colorless
//! Thopter artifact creature token with flying.
//! Disguise {U/R}{U/R}.
//!
//! Disguise is NOT a usable keyword variant (the face-down cast / turn-up
//! mechanic is unmodeled) → keyword line empty, and the "or is turned face
//! up" half of the ETB trigger is unreachable. We wire the enters half via
//! `SelfEntersBattlefield`, minting the Thopter token.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::{KeywordAbility, TokenDefinition};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gadget Technician");
    let goblin = reg.interner_mut().intern("Goblin");
    let artificer = reg.interner_mut().intern("Artificer");
    // Pre-intern the token subtype so the resolver's `lookup("Thopter")`
    // succeeds (catalog idiom — `lookup` is None for never-interned names).
    let _thopter = reg.interner_mut().intern("Thopter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(artificer);

    // GAP: Disguise — face-down cast / turn-face-up mechanic not modeled
    // (keyword line empty); the "or is turned face up" trigger half is
    // therefore unreachable.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_thopter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_thopter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let thopter = reg.interner().lookup("Thopter").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thopter);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: thopter,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
