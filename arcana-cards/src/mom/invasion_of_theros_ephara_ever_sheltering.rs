//! Invasion of Theros // Ephara, Ever-Sheltering
//!
//! Front face: `{2}{W}` Battle — Siege, enters with 6 defense counters.
//! When this Siege enters, search your library for an Aura, God, or Demigod card,
//! reveal it, put it into your hand, then shuffle.
//!
//! Back face: Ephara, Ever-Sheltering — Legendary Enchantment Creature — God (5/7).
//! Ephara has lifelink and indestructible as long as you control at least three
//! other enchantments. (GAP: conditional keyword grant — continuous-effect engine debt.)
//! Whenever another enchantment you control enters, draw a card. (GAP: back-face
//! triggered ability not modeled — back-face triggers only fire while transformed.)
//!
//! GAP: defeat→cast-back-face not auto-wired (CR 310.11).
//! GAP: Ephara's lifelink + indestructible conditional static — continuous-effect engine subsystem.
//! GAP: "whenever another enchantment you control enters, draw a card" — back-face trigger deferred.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Invasion of Theros");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    // Pre-intern subtype strings used at resolve time.
    let _ = reg.interner_mut().intern("Aura");
    let _ = reg.interner_mut().intern("God");
    let _ = reg.interner_mut().intern("Demigod");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ephara, Ever-Sheltering");
    let god_sub = reg.interner_mut().intern("God");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(god_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(7)),
            // GAP: lifelink + indestructible while controlling 3+ other enchantments
            // (static conditional keyword — continuous-effect engine subsystem deferred)
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
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tutor(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Search for an Aura, God, or Demigod card — use subtype OR filter.
    let aura = reg.interner().lookup("Aura");
    let god = reg.interner().lookup("God");
    let demigod = reg.interner().lookup("Demigod");
    let st_ids: Vec<_> = [aura, god, demigod].into_iter().flatten().collect();

    let filter = if st_ids.is_empty() {
        ObjectFilter::new()
    } else {
        ObjectFilter::new().with_subtypes_any(st_ids)
    };

    vec![Effect::TutorToHand {
        player: trig.controller,
        filter,
        reveal: true,
    }]
}
