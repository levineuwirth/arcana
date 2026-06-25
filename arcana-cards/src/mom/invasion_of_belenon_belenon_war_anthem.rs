//! Invasion of Belenon // Belenon War Anthem — `{2}{W}` Battle — Siege
//!
//! Front face (Battle): When this Siege enters, create a 2/2 white and blue
//! Knight creature token with vigilance.
//!
//! Back face (Enchantment): Creatures you control get +1/+1.
//!
//! Defeat-transform to the enchantment back face is auto-wired by the engine SBA
//! (CR 310.11). The back face's "Creatures you control get +1/+1" anthem is
//! installed on the defeat-transform via a Layer-7c controller anthem that is live
//! only while the source shows face 1 (Duration::WhileSourceShowsFace(1)).
//!
//! GAP: Siege protector-designation simplified — any opponent's battle is attackable.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Belenon");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    // Back face: Belenon War Anthem (Enchantment — no mana cost on back face)
    let back_name = reg.interner_mut().intern("Belenon War Anthem");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: None,
            colors: ColorSet::white(),
            types: TypeLine::ENCHANTMENT.into(),
            ..Default::default()
        },
        spell_ability: None,
    };

    // Pre-intern Knight subtype for token resolution lookup
    let _knight_sub = reg.interner_mut().intern("Knight");

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 4,
            })
            .with_transform_back(back)
            // Front ETB: "When this Siege enters, create a 2/2 W/U Knight with vigilance."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face (enchantment): "Creatures you control get +1/+1." Installed
            // on the defeat-transform; live only while showing the back face.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: install_back_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Back-face anthem: creatures the controller controls get +1/+1, live only
/// while the source shows the enchantment (back) face.
fn install_back_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::anthem(
            trig.source,
            trig.controller,
            1,
            1,
            Duration::WhileSourceShowsFace(1),
        ),
    }]
}

fn etb_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let knight = reg.interner().lookup("Knight")
        .expect("Knight interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(knight);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: knight,
            colors: ColorSet::white() | ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Vigilance],
            abilities: vec![],
        },
    }]
}
