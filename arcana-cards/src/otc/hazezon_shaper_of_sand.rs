//! Hazezon, Shaper of Sand — `{R}{G}{W}` 3/3 Legendary Human Warrior.
//! "Desertwalk" — GAP: nonbasic/type landwalk other than the five basic
//!  land subtypes is not supported; emitted with no keyword.
//! "You may play Desert lands from your graveyard." — GAP: alternate
//!  play-from-graveyard permission is not an expressible static.
//! "Whenever a Desert you control enters, create two 1/1 red, green, and
//!  white Sand Warrior creature tokens."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hazezon, Shaper of Sand");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    // Pre-intern the token's subtypes so the resolver can look them up.
    let _sand = reg.interner_mut().intern("Sand");
    let _tok_warrior = reg.interner_mut().intern("Warrior");

    let desert_filter = script::subtype_filter(reg, "Desert")
        .controlled_by(ControllerConstraint::You);

    // GAP: "Desertwalk" — nonbasic landwalk is not supported.
    // GAP: "You may play Desert lands from your graveyard." — no expressible
    // alternate play permission.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: desert_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: make_sand_warriors,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_sand_warriors(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let sand = reg.interner().lookup("Sand").unwrap_or_default();
    let warrior = reg.interner().lookup("Warrior").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sand);
    subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: sand,
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token,
        },
    ]
}
