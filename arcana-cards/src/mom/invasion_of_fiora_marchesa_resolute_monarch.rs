//! Invasion of Fiora // Marchesa, Resolute Monarch
//!
//! Front face: Battle — Siege, `{4}{B}{B}`, enters with 6 defense counters.
//! When this Siege enters, choose one or both —
//!   • Destroy all legendary creatures.
//!   • Destroy all nonlegendary creatures.
//!
//! Back face: Legendary Creature — Human Noble, Menace, Deathtouch.
//! Whenever Marchesa attacks, remove all counters from up to one target permanent.
//! At the beginning of your upkeep, if you haven't been dealt combat damage since
//! your last turn, you draw a card and you lose 1 life.
//!
//! Defeat-transform to the Marchesa creature back face is auto-wired by the
//! engine SBA (CR 310.11); Menace + Deathtouch are intrinsic on the back-face
//! characteristics. Both back-face triggered abilities hit genuine engine
//! blockers and stay GAP'd (the exact missing primitives are named below).
//!
//! GAP: back-face "Whenever Marchesa attacks, remove all counters from up to one
//!      target permanent" — no Effect::RemoveAllCounters variant (only
//!      Effect::RemoveCounters{kind,count} removes a specific counter kind/amount);
//!      "all counters of every kind" is inexpressible. Attack trigger omitted.
//! GAP: back-face "At the beginning of your upkeep, if you haven't been dealt
//!      combat damage since your last turn, draw a card and lose 1 life" — no
//!      per-window "combat damage dealt to you since your last turn" tracker
//!      exists. Upkeep trigger omitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Fiora");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Marchesa, Resolute Monarch");
    let human_sub = reg.interner_mut().intern("Human");
    let noble_sub = reg.interner_mut().intern("Noble");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(human_sub);
    back_subtypes.0.insert(noble_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            keywords: vec![KeywordAbility::Menace, KeywordAbility::Deathtouch],
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 6,
            })
            .with_transform_back(back)
            // ETB: choose one or both — destroy all legendary / destroy all nonlegendary.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: back-face attack trigger (remove ALL counters) inexpressible — no
            //      Effect::RemoveAllCounters. Back-face upkeep trigger inexpressible —
            //      no "dealt combat damage since your last turn" tracker. See module doc.
    )
}

fn etb_modal(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Mode A: destroy all legendary creatures.
    let legendary_filter = ObjectFilter::creature()
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY));
    let legendary_ids = arcana_core::script::ids_matching(state, &legendary_filter, trig.controller);

    // Mode B: destroy all nonlegendary creatures.
    let nonlegendary_filter = ObjectFilter::creature()
        .without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY));
    let nonlegendary_ids = arcana_core::script::ids_matching(state, &nonlegendary_filter, trig.controller);

    // Both modes: destroy all legendary then all nonlegendary.
    let mut effects = Vec::new();
    effects.push(Effect::ForEach {
        targets: legendary_ids,
        effect: Box::new(Effect::DestroyPermanent {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    });
    effects.push(Effect::ForEach {
        targets: nonlegendary_ids,
        effect: Box::new(Effect::DestroyPermanent {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    });
    effects
}
