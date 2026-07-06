//! Archfiend's Vessel — `{B}` 1/1 Human Cleric with Lifelink.
//! "When this creature enters, if it entered from your graveyard or you cast it
//! from your graveyard, exile it. If you do, create a 5/5 black Demon creature
//! token with flying."
//!
//! Lifelink is a base keyword. The ETB trigger fires on any enter, but the
//! effect self-gates on `PendingTrigger::entered_from_zone()` being a graveyard
//! (reanimation) — a normally-cast Vessel enters from the stack and makes no
//! Demon. GAP: "or you cast it from your graveyard" (a rare enabler-only case)
//! isn't distinguished, since a cast permanent enters from the stack either way.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::TokenDefinition;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archfiend's Vessel");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let _demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            // "if it entered from your graveyard" is gated inside the effect via
            // PendingTrigger::entered_from_zone() (intervening-if predicates only
            // see game state, not the triggering event's origin zone).
            intervening_if: None,
            effect: etb_exile_make_demon,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_exile_make_demon(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "if it entered from your graveyard": only reanimation enters from a
    // graveyard — a normally-cast Vessel enters from the stack, so no Demon.
    if !matches!(trig.entered_from_zone(), Some(Zone::Graveyard(_))) {
        return Vec::new();
    }
    let demon = reg.interner().lookup("Demon").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    vec![
        Effect::ExilePermanent { target: trig.source },
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: demon,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(5)),
                toughness: Some(PtValue::Fixed(5)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use arcana_core::events::GameEvent;

    fn etb_trigger(from: Zone) -> PendingTrigger {
        PendingTrigger {
            source: 1,
            trigger_id: 1,
            controller: 0,
            trigger_event: GameEvent::EntersBattlefield {
                object_id: 1,
                from_zone: from,
                was_cast: !matches!(from, Zone::Graveyard(_)),
            },
            targets: Default::default(),
            effect_override: None,
        }
    }

    #[test]
    fn demon_token_only_when_entering_from_graveyard() {
        let mut reg = CardRegistry::new();
        register(&mut reg);
        let s = GameState::new(2, 0);

        // Reanimated (enters from the graveyard): exile it + create the Demon.
        let reanimated = etb_exile_make_demon(&s, &etb_trigger(Zone::Graveyard(0)), &reg);
        assert_eq!(reanimated.len(), 2,
            "from graveyard → ExilePermanent + CreateToken");

        // Normally cast (enters from the stack): no exile, no Demon.
        let cast = etb_exile_make_demon(&s, &etb_trigger(Zone::Stack), &reg);
        assert!(cast.is_empty(),
            "cast from hand enters from the stack → no Demon");
    }
}
