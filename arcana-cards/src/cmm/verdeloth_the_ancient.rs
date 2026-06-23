//! Verdeloth the Ancient — `{4}{G}{G}` Legendary 4/7 Treefolk.
//! "Kicker {X}. Saproling creatures and other Treefolk creatures get +1/+1.
//! When Verdeloth enters, if it was kicked, create X 1/1 green Saproling
//! creature tokens."
//!
//! * Kicker {X} — // GAP: not a usable KeywordAbility variant; the X payment
//!   is not queryable.
//! * Saproling creatures and other Treefolk creatures get +1/+1.
//!   Wired via a SelfEntersBattlefield trigger installing a
//!   `ContinuousEffect::filtered_pump` over creatures whose subtype is
//!   Saproling OR Treefolk (any controller — no "you control" qualifier),
//!   +1/+1, lasting while this creature is on the battlefield. (The "other"
//!   qualifier on Treefolk is a documented minor fidelity gap — Verdeloth is
//!   herself a Treefolk and matches the base-characteristics filter.)
//! * When Verdeloth enters, if it was kicked, create X 1/1 green Saproling
//!   tokens. // GAP: kicked status and the X paid are not queryable, so the
//!   conditional ETB token creation cannot be expressed.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Verdeloth the Ancient");
    let treefolk = reg.interner_mut().intern("Treefolk");
    // Intern Saproling now so the resolver can look it up read-only.
    let _saproling = reg.interner_mut().intern("Saproling");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);

    // GAP: Kicker {X} — not a usable KeywordAbility variant.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    // GAP: "When Verdeloth enters, if it was kicked, create X 1/1 green
    // Saproling tokens" — kicked status and the X paid are not queryable.

    reg.register(
        CardDefinition::new(name, chars)
            // "Saproling creatures and other Treefolk creatures get +1/+1."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_treefolk_saproling_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn install_treefolk_saproling_anthem(
    _s: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let saproling = reg.interner().lookup("Saproling").expect("Saproling interned at register");
    let treefolk = reg.interner().lookup("Treefolk").expect("Treefolk interned at register");
    let filter = ObjectFilter::creature().with_subtypes_any(vec![saproling, treefolk]);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            filter,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
