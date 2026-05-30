//! Birgi, God of Storytelling // Harnfel, Horn of Bounty — `{2}{R}` Legendary
//! Creature — God 3/3 (MDFC).
//!
//! Front face (Birgi): Whenever you cast a spell, add {R}. Until end of turn,
//! you don't lose this mana as steps and phases end. Creatures you control can
//! boast twice during each of your turns rather than once.
//!
//! Back face (Harnfel, Horn of Bounty): Legendary Artifact. Discard a card:
//! Exile the top two cards of your library. You may play those cards this turn.
//!
//! GAP: "Until end of turn, you don't lose this mana as steps and phases end"
//! (floating mana persistence) is not expressible.
//! GAP: "Creatures you control can boast twice" (boast frequency modifier) is
//! not expressible.
//! GAP: Back-face activated ability "Discard a card: Exile top two cards. You
//! may play those cards this turn" — the discard-as-cost activation is GAP
//! (OptionalPaymentKind has no Discard variant); back-face-only triggered
//! abilities not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Birgi, God of Storytelling");
    let god_sub = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Back face: Harnfel, Horn of Bounty — Legendary Artifact
    let back_name = reg.interner_mut().intern("Harnfel, Horn of Bounty");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
            colors: ColorSet::red(),
            types: TypeLine::ARTIFACT.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back)
            // Front face trigger: whenever you cast a spell, add {R}
            // GAP: floating mana persistence ("don't lose this mana") not modeled
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: spell_cast_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: back-face-only activated ability (Discard a card: ...) not modeled
    )
}

fn spell_cast_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, trig.source)],
    }]
}
