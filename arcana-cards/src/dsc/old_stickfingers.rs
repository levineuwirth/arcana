//! Old Stickfingers — `{X}{B}{G}` */* Legendary black-green Horror.
//!
//! Oracle:
//! * "When you cast this spell, reveal cards from the top of your library
//!   until you reveal X creature cards. Put all creature cards revealed
//!   this way into your graveyard, then put the rest on the bottom of your
//!   library in a random order." — GAP: there is no "when you cast this
//!   spell" self-cast trigger condition, and `RevealUntil` finds only the
//!   FIRST matching card (it can't reveal-until-X with X from the cast),
//!   so the whole cast ability is unexpressible.
//! * "Old Stickfingers's power and toughness are each equal to the number
//!   of creature cards in your graveyard." — a characteristic-defining
//!   ability (symmetric `*/*`), wired at Layer 7a via a
//!   SelfEntersBattlefield self_pt_cda whose compute counts creature cards
//!   in your graveyard and returns `(n, n)`. (`self_pt_from_match` counts
//!   BATTLEFIELD permanents, not graveyard cards, so the scalar compute fn
//!   is used.) Bones use `PtValue::Star`.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Old Stickfingers");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    // GAP: cast-trigger — no self-cast trigger, and reveal-until-X (X from
    // cast cost) is not expressible via RevealUntil (first-match only).
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

// Power and toughness each = creature cards in your graveyard.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = s
        .objects
        .objects_in_zone(Zone::Graveyard(who))
        .filter(|o| o.characteristics.types.is_creature())
        .count() as i32;
    (n, n)
}
