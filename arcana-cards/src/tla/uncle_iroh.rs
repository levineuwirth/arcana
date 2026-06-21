//! Uncle Iroh — `{1}{R/G}{R/G}` 4/2 Legendary Creature — Human Noble Ally.
//! "Firebending 1 (Whenever this creature attacks, add {R}. This mana
//!   lasts until end of combat.)"
//! "Lesson spells you cast cost {1} less to cast."
//!
//! Firebending is not a usable `KeywordAbility`; its reminder text is a
//! "whenever this attacks, add {R}" trigger, modeled below via AddMana
//! (the "lasts until end of combat" rider is not expressible and is a
//! fidelity GAP). The Lesson cost-reduction static has no cost-reduction
//! primitive and is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Uncle Iroh");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R/G}{R/G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static — "Lesson spells you cast cost {1} less to cast." No
    // cost-reduction primitive available for this card class.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: firebending_add_red,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn firebending_add_red(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP (fidelity): the "lasts until end of combat" rider is not
    // expressible; the mana is added normally.
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, trig.source)],
    }]
}
