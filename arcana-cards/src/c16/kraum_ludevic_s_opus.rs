//! Kraum, Ludevic's Opus — `{3}{U}{R}` Legendary 4/4 Zombie Horror with
//! Flying and Haste.
//! Whenever an opponent casts their second spell each turn, draw a card.
//! Partner — GAP (Commander-only deckbuilding static; not in the usable
//! keyword surface).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kraum, Ludevic's Opus");
    let zombie = reg.interner_mut().intern("Zombie");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // Partner is GAP (Commander static, not a usable KeywordAbility).
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: opponent_second_spell_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn opponent_second_spell_draw(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "their second spell each turn" — gate on the triggering opponent having
    // cast exactly two spells this turn (the just-cast spell is counted).
    let Some(caster) = trig.triggering_caster() else {
        return Vec::new();
    };
    let cast = script::spells_cast_this_turn(state, &ObjectFilter::default(), caster);
    if cast != 2 {
        return Vec::new();
    }
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
