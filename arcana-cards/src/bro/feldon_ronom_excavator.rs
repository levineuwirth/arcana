//! Feldon, Ronom Excavator — `{1}{R}` 2/2 Legendary Human Artificer.
//! Haste.
//! Feldon can't block.
//! Whenever Feldon is dealt damage, exile that many cards from the top of
//! your library. Choose one of them. Until the end of your next turn, you
//! may play that card.
//!
//! "Feldon can't block" is a static restriction with no engine hook — GAP'd.
//! The dealt-damage trigger is implemented with Effect::ImpulseExile of
//! N = damage taken (closest primitive: it exiles N cards with play-
//! permission; "choose one of them" / the until-end-of-next-turn window are
//! a fidelity gap vs ImpulseExile's "play any of them this turn").

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feldon, Ronom Excavator");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP static: "Feldon can't block" — no can't-block restriction hook.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfIsDealtDamage { combat_only: false },
            intervening_if: None,
            effect: impulse_dealt,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn impulse_dealt(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::ImpulseExile {
        player: trig.controller,
        count: n,
    }]
}
